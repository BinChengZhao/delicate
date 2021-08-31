use super::prelude::*;
use db::schema::executor_processor;

pub(crate) async fn loop_health_check(pool: ShareData<db::ConnectionPool>) {
    let mut interval = interval(Duration::from_secs(20));
    loop {
        interval.tick().await;
        if let Ok(conn) = pool.get() {
            health_check(conn)
                .await
                .map_err(|e| error!(target:"loop-health-check", "{}", e.to_string()))
                .ok();
            continue;
        }

        error!(target:"loop-health-check", "No available database connection.");
    }
}

async fn health_check(conn: db::PoolConnection) -> Result<(), CommonError> {
    let executor_packages = web::block::<_, _, diesel::result::Error>(move || {
        executor_processor::table
            .select((executor_processor::host, executor_processor::token))
            .filter(executor_processor::status.eq(state::executor_processor::State::Enabled as i16))
            .load::<(String, String)>(&conn)
    })
    .await?
    .into_iter();

    let request_all: JoinAll<SendClientRequest> = executor_packages
        .filter_map(|(executor_host, executor_token)| {
            let message = delicate_utils_executor_processor::HealthScreenUnit::default();

            let executor_host =
                "http://".to_string() + (executor_host.deref()) + "/api/executor/health_screen";

            message
                .sign(Some(&executor_token))
                .map(|s| (s, executor_host))
                .ok()
        })
        .map(|(signed_health_screen_unit, executor_host)| {
            RequestClient::builder()
                .finish()
                .post(executor_host)
                .send_json(&signed_health_screen_unit)
        })
        .collect::<Vec<SendClientRequest>>()
        .into_iter()
        .collect();

    let _span_ = span!(Level::INFO, "health-check").entered();
    handle_response::<UnifiedResponseMessages<delicate_utils_health_check::HealthCheckPackage>>(
        request_all,
    )
    .await;
    Ok(())
}
