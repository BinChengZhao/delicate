use super::prelude::*;
use db::schema::{operation_log, operation_log_detail};

lazy_static! {
    pub static ref OPERATION_LOG_CONSUMERS: (
        AsyncSender<NewOperationLogPair>,
        AsyncReceiver<NewOperationLogPair>
    ) = async_channel::unbounded::<NewOperationLogPair>();
}

// Operation log asynchronous consumer
//
// The user's operations in the system are logged to track,
// But in order not to affect the performance of the system,
// These logs go through the channel with the asynchronous state machine to consume.
pub(crate) async fn loop_operate_logs(pool: ShareData<db::ConnectionPool>) {
    loop {
        let mut operation_log_pairs: Vec<NewOperationLogPair> = Vec::new();

        'logs_collection: for _ in 0..512 {
            let logs_future: RtTimeout<_> =
                rt_timeout(Duration::from_secs(3), OPERATION_LOG_CONSUMERS.1.recv());

            match logs_future.await {
                // No new events and timeout.
                Err(_) => break 'logs_collection,
                // Internal runtime exception.
                Ok(Err(_)) => {
                    return;
                }
                Ok(Ok(log_pair)) => {
                    operation_log_pairs.push(log_pair);
                }
            }
        }

        if operation_log_pairs.is_empty() {
            continue;
        }

        if let Ok(conn) = pool.get() {
            operate_logs(conn, operation_log_pairs)
                .await
                .map_err(|e| error!("operate-logs: {}", e))
                .ok();
            continue;
        }

        error!(target:"loop-operate-logs", "No available database connection.");
    }
}

pub(crate) async fn send_option_operation_log_pair(
    option_operation_log_pair: NewOperationLogPairOption,
) {
    if let Some(log_pair) = option_operation_log_pair {
        OPERATION_LOG_CONSUMERS.0.send(log_pair).await.ok();
    }
}

async fn operate_logs(
    conn: db::PoolConnection,
    logs: Vec<NewOperationLogPair>,
) -> Result<(), CommonError> {
    let mut operation_logs = Vec::new();
    let mut operation_log_details = Vec::new();

    logs.into_iter()
        .for_each(|(operation_log, operation_log_detail)| {
            operation_logs.push(operation_log);
            operation_log_details.push(operation_log_detail);
        });
    web::block::<_, _, diesel::result::Error>(move || {
        // Use this solution to ensure that the conditions are met (innodb_autoinc_lock_mode = 1 ("executive" lock mode)):
        // https://stackoverflow.com/questions/27225804/mysql-batch-insert-on-multiple-tables-with-last-insert-id
        // https://dev.mysql.com/doc/refman/8.0/en/innodb-auto-increment-handling.html
        // https://dev.mysql.com/doc/refman/5.6/en/information-functions.html#function_last-insert-id

        diesel::insert_into(operation_log::table)
            .values(&operation_logs)
            .execute(&conn)?;

        let last_id = diesel::select(db::last_insert_id).get_result::<u64>(&conn)?;

        operation_log_details
            .iter_mut()
            .scan(last_id, |last_id, detail| {
                detail.operation_log_id = *last_id;
                *last_id += 1;
                Some(())
            })
            .for_each(drop);

        diesel::insert_into(operation_log_detail::table)
            .values(&operation_log_details)
            .execute(&conn)?;
        Ok(())
    })
    .await?;

    Ok(())
}
