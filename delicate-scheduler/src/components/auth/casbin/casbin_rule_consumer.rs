use crate::prelude::*;
use futures::StreamExt as _;
use redis::aio::Connection;
use redis::{AsyncCommands, RedisResult};

// `CASBIN_EVENT_CONSUMERS` is the internal channel that processes the Casbin-event on the current machine,
// Inputs data from the casbin-watcher, and then publishes it to redis from the coroutine.

// `delicate:auth:casbin:rules:sync` is the data structure in redis,
// The middleware responsible for multiple machine publish / subscriptions.
lazy_static! {
    pub static ref CASBIN_EVENT_CONSUMERS: (AsyncSender<CasbinEventData>, AsyncReceiver<CasbinEventData>) =
        async_channel::unbounded::<CasbinEventData>();
}

#[allow(dead_code)]
#[inline(always)]
pub(crate) fn handle_event_for_watcher(event: CasbinEventData) {
    rt_spawn(async move {
        CASBIN_EVENT_CONSUMERS
            .0
            .send(event)
            .await
            .map_err(|e| error!("handle_event_for_watcher:send:fail {}", e))
            .ok();
    });
}

// Casbin event asynchronous consumer
//
// User adjustments to permissions are notified to
// Each machine in the `delicate-schduler` cluster
// Through the redis publish-subscribe mechanism.
#[allow(dead_code)]
pub(crate) async fn loop_sync_casbin_rules(_enforcer: ShareData<RwLock<Enforcer>>) {
    let redis_url = env::var("REDIS_URL").expect("The redis url could not be acquired.");
    let client = redis::Client::open(redis_url)
        .expect("The redis client resource could not be initialized.");

    loop {
        let publish_conn = client.get_async_connection();
        let pubsub_conn = client.get_async_connection();
        let conn_pair = join(publish_conn, pubsub_conn).await;

        if let (Ok(publish_conn), Ok(_conn)) = conn_pair {
            publish_casbin_rule_events(publish_conn).await.ok();
            continue;
        }

        error!(target:"loop-sync-casbin-rules", "No available redis connection.");
    }
}

pub(crate) async fn publish_casbin_rule_events(mut publish_conn: Connection) -> RedisResult<()> {
    if let Ok(_casbin_event) = CASBIN_EVENT_CONSUMERS.1.recv().await {
        // Serialize the _casbin_event ,then published it.
        publish_conn
            .publish("delicate:auth:casbin:rules:sync", "banana")
            .await?;
    }

    Ok(())
}

#[allow(dead_code)]
pub(crate) async fn subscribe_casbin_rule_events(
    conn: Connection,
    enforcer: ShareData<RwLock<Enforcer>>,
) -> RedisResult<()> {
    let mut pubsub_conn = conn.into_pubsub();

    pubsub_conn
        .subscribe("delicate:auth:casbin:rules:sync")
        .await?;
    let mut pubsub_stream = pubsub_conn.on_message();

    while let Some(pubsub_msg) = pubsub_stream.next().await {
        let msg: String = pubsub_msg.get_payload()?;
        // Deserialize the msg ,then deal with it.
        enforcer
            .write()
            .await
            .add_permission_for_user("a", Vec::new())
            .await
            .map_err(|e| {
                error!(
                    "subscribe_casbin_rule_events:add_permission_for_user:fail: {}",
                    e
                )
            })
            .ok();

        dbg!(msg);
    }

    Ok(())
}
