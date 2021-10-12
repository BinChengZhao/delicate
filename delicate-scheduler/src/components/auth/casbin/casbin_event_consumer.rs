use crate::prelude::*;
use futures::StreamExt as _;
use redis::aio::Connection;
use redis::{AsyncCommands, RedisResult};

// `DELICATE_AUTH_RULE_EVENT_CONSUMERS` is the internal channel that processes the Casbin-event on the current machine,
// Inputs data from the casbin-watcher, and then publishes it to redis from the coroutine.

// `delicate:auth:casbin:rules:sync` is the data structure in redis,
// The middleware responsible for multiple machine publish / subscriptions.
lazy_static! {
    pub(crate) static ref DELICATE_AUTH_RULE_EVENT_CONSUMERS: (
        AsyncSender<DelicateAuthRuleEvent>,
        AsyncReceiver<DelicateAuthRuleEvent>
    ) = async_channel::unbounded::<DelicateAuthRuleEvent>();
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct DelicateAuthRuleEvent {
    auth_adapter_event: AuthAdapterEventModel,
}

impl DelicateAuthRuleEvent {
    pub(crate) fn casbin_event(casbin_adapter_event: CasbinEventModel) -> Self {
        let auth_adapter_event = AuthAdapterEventModel::Casbin(casbin_adapter_event);
        DelicateAuthRuleEvent { auth_adapter_event }
    }

    pub(crate) async fn consume_by_consumer(self, consumer: &mut Enforcer) {
        match self.auth_adapter_event {
            AuthAdapterEventModel::Casbin(casbin_event) => {
                Self::consume_by_casbin_consumer(casbin_event, consumer).await
            }
        }
    }

    pub(crate) async fn consume_by_casbin_consumer(
        CasbinEventModel {
            operation,
            sec,
            ptype,
            dynamic_fields,
        }: CasbinEventModel,
        consumer: &mut Enforcer,
    ) {
        // When the rest of the machines receive the notification,
        // They need to turn off `enable_auto_notify_watcher`
        // Before consuming casbin-event's data to
        // Avoid re-publish the same data over and over again.
        // And avoid re-save in db over and over again.

        consumer.enable_auto_notify_watcher(false);
        consumer.enable_auto_save(false);

        match operation {
            AuthAdapterEventOperation::AddPolicy => match dynamic_fields {
                CasbinDynamicField::Singlelayer(dynamic_fields) => {
                    consumer
                        .add_policy_internal(&sec, &ptype, dynamic_fields)
                        .await
                }

                CasbinDynamicField::MultiLayer(dynamic_fields) => {
                    consumer
                        .add_policies_internal(&sec, &ptype, dynamic_fields)
                        .await
                }
            },
            AuthAdapterEventOperation::RemovePolicy => match dynamic_fields {
                CasbinDynamicField::Singlelayer(dynamic_fields) => {
                    consumer
                        .remove_policy_internal(&sec, &ptype, dynamic_fields)
                        .await
                }

                CasbinDynamicField::MultiLayer(dynamic_fields) => {
                    consumer
                        .remove_policies_internal(&sec, &ptype, dynamic_fields)
                        .await
                }
            },
        }
        .map_err(|e| error!("consume_by_casbin_consumer:fail: {}", e))
        .ok();

        // Restore `enable_auto_notify_watcher`.
        consumer.enable_auto_notify_watcher(true);
        consumer.enable_auto_save(true);
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum AuthAdapterEventModel {
    Casbin(CasbinEventModel),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum AuthAdapterEventOperation {
    AddPolicy,
    RemovePolicy,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct CasbinEventModel {
    operation: AuthAdapterEventOperation,
    sec: String,
    ptype: String,
    dynamic_fields: CasbinDynamicField,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum CasbinDynamicField {
    Singlelayer(Vec<String>),
    MultiLayer(Vec<Vec<String>>),
}
#[allow(dead_code)]
#[inline(always)]
pub(crate) fn handle_event_for_watcher(event: CasbinEventData) {
    let delicate_auth_rule_event = match event {
        EventData::AddPolicy(sec, ptype, dynamic_fields) => {
            let operation = AuthAdapterEventOperation::AddPolicy;
            let dynamic_fields = CasbinDynamicField::Singlelayer(dynamic_fields);
            let casbin_event_model = CasbinEventModel {
                operation,
                sec,
                ptype,
                dynamic_fields,
            };
            DelicateAuthRuleEvent::casbin_event(casbin_event_model)
        }
        EventData::RemovePolicy(sec, ptype, dynamic_fields) => {
            let operation = AuthAdapterEventOperation::RemovePolicy;
            let dynamic_fields = CasbinDynamicField::Singlelayer(dynamic_fields);
            let casbin_event_model = CasbinEventModel {
                operation,
                sec,
                ptype,
                dynamic_fields,
            };
            DelicateAuthRuleEvent::casbin_event(casbin_event_model)
        }
        EventData::AddPolicies(sec, ptype, dynamic_fields) => {
            let operation = AuthAdapterEventOperation::AddPolicy;
            let dynamic_fields = CasbinDynamicField::MultiLayer(dynamic_fields);

            let casbin_event_model = CasbinEventModel {
                operation,
                sec,
                ptype,
                dynamic_fields,
            };
            DelicateAuthRuleEvent::casbin_event(casbin_event_model)
        }
        EventData::RemovePolicies(sec, ptype, dynamic_fields) => {
            let operation = AuthAdapterEventOperation::RemovePolicy;
            let dynamic_fields = CasbinDynamicField::MultiLayer(dynamic_fields);
            let casbin_event_model = CasbinEventModel {
                operation,
                sec,
                ptype,
                dynamic_fields,
            };
            DelicateAuthRuleEvent::casbin_event(casbin_event_model)
        }
        _ => {
            return;
        }
    };

    rt_spawn(async move {
        DELICATE_AUTH_RULE_EVENT_CONSUMERS
            .0
            .send(delicate_auth_rule_event)
            .await
            .map_err(|e| error!("handle_event_for_watcher:send:fail {}", e))
            .ok();
    });
}

/// Casbin event asynchronous consumer
///
/// User adjustments to permissions are notified to
/// Each machine in the `delicate-schduler` cluster
/// Through the redis publish-subscribe mechanism.
#[allow(dead_code)]
pub(crate) fn launch_casbin_rule_events_consumer(
    redis_client: redis::Client,
    enforcer: ShareData<RwLock<Enforcer>>,
) {
    rt_spawn(loop_publish_casbin_rule_events(redis_client.clone()));
    rt_spawn(loop_subscribe_casbin_rule_events(redis_client, enforcer));
}

pub(crate) async fn loop_publish_casbin_rule_events(redis_client: redis::Client) {
    loop {
        let publish_conn_result = redis_client.get_async_connection().await;

        if let Ok(publish_conn) = publish_conn_result {
            publish_casbin_rule_events(publish_conn).await.ok();
            continue;
        }

        error!(target:"loop-publish-casbin-rule-events", "No available redis connection.");
        rt_delay_for(Duration::from_secs(1)).await;
    }
}

pub(crate) async fn publish_casbin_rule_events(mut publish_conn: Connection) -> RedisResult<()> {
    while let Ok(casbin_event) = DELICATE_AUTH_RULE_EVENT_CONSUMERS.1.recv().await {
        // Serialize the _casbin_event ,then published it.
        if let Ok(msg) = to_json_string(&casbin_event) {
            publish_conn
                .publish("delicate:auth:casbin:rules:sync", &msg)
                .await?;
        }
    }

    Ok(())
}

pub(crate) async fn loop_subscribe_casbin_rule_events(
    redis_client: redis::Client,
    enforcer: ShareData<RwLock<Enforcer>>,
) {
    loop {
        let pubsub_conn_result = redis_client.get_async_connection().await;

        if let Ok(pubsub_conn) = pubsub_conn_result {
            subscribe_casbin_rule_events(pubsub_conn, enforcer.clone())
                .await
                .ok();
            continue;
        }

        error!(target:"loop-subscribe-casbin-rule-events", "No available redis connection.");
        rt_delay_for(Duration::from_secs(1)).await;
    }
}

pub(crate) async fn subscribe_casbin_rule_events(
    conn: Connection,
    enforcer: ShareData<RwLock<Enforcer>>,
) -> Result<(), CommonError> {
    let mut pubsub_conn = conn.into_pubsub();

    pubsub_conn
        .subscribe("delicate:auth:casbin:rules:sync")
        .await?;
    let mut pubsub_stream = pubsub_conn.on_message();

    // TODO: The machine itself does not consume the messages it publishes.
    while let Some(pubsub_msg) = pubsub_stream.next().await {
        // Deserialize the msg ,then deal with it.
        let msg: String = pubsub_msg.get_payload()?;
        debug!("subscribe_casbin_rule_events: {}", &msg);

        let auth_rule_event: DelicateAuthRuleEvent = from_json_str(&msg)?;
        let mut enforcer = enforcer.write().await;
        auth_rule_event.consume_by_consumer(&mut enforcer).await;
    }

    Ok(())
}
