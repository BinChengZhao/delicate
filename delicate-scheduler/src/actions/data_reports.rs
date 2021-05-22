use super::prelude::*;

pub(crate) fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(show_one_day_tasks_state);
}

#[get("/api/tasks_state/one_day")]
async fn show_one_day_tasks_state(pool: ShareData<db::ConnectionPool>) -> HttpResponse {
    use db::schema::task_log;
    use state::task_log::State;

    if let Ok(conn) = pool.get() {
        return HttpResponse::Ok().json(
            Into::<UnifiedResponseMessages<Vec<model::TaskState>>>::into(
                web::block::<_, _, diesel::result::Error>(move || {
                    let now = NaiveDateTime::from_timestamp(get_timestamp() as i64, 0);
                    let past_day = now - ChronoDuration::days(1);

                    // TODO: Optimize it.
                    // the number of tasks started in a given hour ( By created_time ), the
                    // the number of tasks that ended normally at a certain time
                    // the number of tasks that ended abnormally at a certain time
                    // the number of tasks that timed out at a certain time
                    // Number of manually cancelled tasks at a certain time
                    // Update to add a limit so that only tasks with Running status can be modified.

                    // Fix-`Count` by: https://github.com/diesel-rs/diesel/issues/1781.
                    let mut create_count: Vec<model::TaskState> = task_log::table
                        .select(&(
                            diesel::dsl::sql::<diesel::sql_types::SmallInt>(
                                "Hour(task_log.created_time) as hour_num",
                            ),
                            task_log::status,
                            diesel::dsl::sql::<diesel::sql_types::BigInt>(
                                "count(task_log.id) as total",
                            ),
                        ))
                        .filter(task_log::created_time.between(past_day, now))
                        .group_by(diesel::dsl::sql::<()>("hour_num"))
                        .load(&conn)?;

                    create_count
                        .iter_mut()
                        .for_each(|s| s.status = State::Running as i16);

                    let end_states_count: Vec<model::TaskState> = task_log::table
                        .select(&(
                            diesel::dsl::sql::<diesel::sql_types::SmallInt>(
                                "Hour(task_log.updated_time) as hour_num",
                            ),
                            task_log::status,
                            diesel::dsl::sql::<diesel::sql_types::BigInt>(
                                "count(task_log.id) as total",
                            ),
                        ))
                        .filter(task_log::updated_time.between(past_day, now))
                        .filter(task_log::status.eq_any(&[
                            State::AbnormalEnding as i16,
                            State::NormalEnding as i16,
                            State::TimeoutEnding as i16,
                            State::TmanualCancellation as i16,
                        ]))
                        .group_by((diesel::dsl::sql::<()>("hour_num"), task_log::status))
                        .load(&conn)?;

                    let all_task_state_counts: Vec<model::TaskState> = create_count
                        .into_iter()
                        .chain(end_states_count.into_iter())
                        .collect();

                    Ok(all_task_state_counts)
                })
                .await,
            ),
        );
    }

    HttpResponse::Ok().json(UnifiedResponseMessages::<Vec<model::TaskState>>::error())
}
