use super::prelude::*;
use db::schema::executor_processor;

// FIXME:
pub(crate) async fn loop_health_check(
    pool: Arc<db::ConnectionPool>,
    request_client: RequestClient,
) {
    let mut interval = interval(Duration::from_secs(20));
    loop {
        interval.tick().await;
        if let Ok(conn) = pool.get() {
            health_check(conn, request_client.clone())
                .await
                .map_err(|e| error!(target:"loop-health-check", "{}", e.to_string()))
                .ok();
            continue;
        }

        error!(target:"loop-health-check", "No available database connection.");
    }
}

async fn health_check(
    conn: db::PoolConnection,
    request_client: RequestClient,
) -> Result<(), CommonError> {
    let (executor_packages, conn) =
        spawn_blocking::<_, Result<_, diesel::result::Error>>(move || {
            let executors = executor_processor::table
                .select((
                    executor_processor::id,
                    executor_processor::host,
                    executor_processor::token,
                ))
                .filter(
                    executor_processor::status.eq(state::executor_processor::State::Enabled as i16),
                )
                .load::<(i64, String, String)>(&conn)?;

            Ok((executors, conn))
        })
        .await??;
    let all_executor_ids: HashSet<i64> = executor_packages.iter().map(|(id, _, _)| *id).collect();

    // FIXME:
    todo!();
    // let request_all: JoinAll<_> = executor_packages
    //     .into_iter()
    //     .filter_map(|(_, executor_host, executor_token)| {
    //         let message = delicate_utils_executor_processor::HealthScreenUnit::default();

    //         let executor_host =
    //             "http://".to_string() + (executor_host.deref()) + "/api/executor/health_screen";

    //         message
    //             .sign(Some(&executor_token))
    //             .map(|s| (s, executor_host))
    //             .ok()
    //     })
    //     .map(|(signed_health_screen_unit, executor_host)| {
    //         request_client
    //             .post(executor_host)
    //             .json(&signed_health_screen_unit)
    //             .send()
    //     })
    //     .collect::<Vec<_>>()
    //     .into_iter()
    //     .collect();

    // let health_check_packages = handle_response::<
    //     UnifiedResponseMessages<delicate_utils_health_check::HealthCheckPackage>,
    // >(request_all)
    // .instrument(span!(Level::INFO, "health-check"))
    // .await;

    // let health_processors: HashSet<i64> = health_check_packages
    //     .iter()
    //     .map(|e| e.get_data_ref().bind_request.executor_processor_id)
    //     .collect();

    // let abnormal_processor: Vec<i64> = all_executor_ids
    //     .difference(&health_processors)
    //     .copied()
    //     .collect();

    // if !abnormal_processor.is_empty() {
    //     spawn_blocking::<_, Result<_, diesel::result::Error>>(move || {
    //         diesel::update(
    //             executor_processor::table
    //                 .filter(executor_processor::id.eq_any(&abnormal_processor[..])),
    //         )
    //         .set(executor_processor::status.eq(state::executor_processor::State::Abnormal as i16))
    //         .execute(&conn)
    //     })
    //     .await?;
    // }

    // Ok(())
}
