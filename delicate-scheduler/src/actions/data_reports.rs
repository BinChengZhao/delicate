use super::prelude::*;

pub(crate) fn config_route(route: Route) -> Route {
    route.at("/api/tasks_state/one_day", get(show_one_day_tasks_state))
}

#[handler]
async fn show_one_day_tasks_state(pool: Data<&db::ConnectionPool>) -> impl IntoResponse {
    use db::schema::task_log;
    use state::task_log::State;

    if let Ok(conn) = pool.get() {
        let daily_state_result = spawn_blocking::<_, Result<_, diesel::result::Error>>(move || {
            let datetime: DateTime<Local> = SystemTime::now().into();
            let now = datetime.naive_local();
            let raw_past_day = now - ChronoDuration::days(1);
            let start_hour = datetime.hour() + 1;

            let past_day = raw_past_day
                .with_hour(start_hour)
                .map(|t| t.with_minute(0).map(|t| t.with_second(0)))
                .flatten()
                .flatten()
                .unwrap_or(raw_past_day);

            let hours_range: Vec<u32> = (start_hour..).take(24).map(|n| n % 24).collect();

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
                    diesel::dsl::sql::<diesel::sql_types::BigInt>("count(task_log.id) as total"),
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
                    diesel::dsl::sql::<diesel::sql_types::BigInt>("count(task_log.id) as total"),
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

            Ok(pre_show_one_day_tasks_state(
                hours_range,
                create_count,
                end_states_count,
            ))
        })
        .await;
        let daily_state = daily_state_result
            .map(|daily_state_result| {
                Into::<UnifiedResponseMessages<model::DailyState>>::into(daily_state_result)
            })
            .unwrap_or_else(|e| {
                UnifiedResponseMessages::<model::DailyState>::error()
                    .customized_error_msg(e.to_string())
            });
        return Json(daily_state);
    }

    Json(UnifiedResponseMessages::<model::DailyState>::error())
}

pub(crate) fn pre_show_one_day_tasks_state(
    hours_range: Vec<u32>,
    create_count: Vec<model::TaskState>,
    end_states_count: Vec<model::TaskState>,
) -> model::DailyState {
    use state::task_log::State;

    let mut daily_states = model::DailyState::default();

    let mut created_map: HashMap<u32, i64> = create_count
        .iter()
        .map(|t| (t.hour_num as u32, t.total))
        .collect();
    let mut timeout_map: HashMap<u32, i64> = HashMap::default();
    let mut finished_map: HashMap<u32, i64> = HashMap::default();
    let mut abnormal_map: HashMap<u32, i64> = HashMap::default();
    let mut canceled_map: HashMap<u32, i64> = HashMap::default();
    end_states_count
        .iter()
        .map(|t| match Into::<State>::into(t.status) {
            State::TimeoutEnding => {
                timeout_map.insert(t.hour_num as u32, t.total);
            }
            State::NormalEnding => {
                finished_map.insert(t.hour_num as u32, t.total);
            }
            State::AbnormalEnding => {
                abnormal_map.insert(t.hour_num as u32, t.total);
            }
            State::TmanualCancellation => {
                canceled_map.insert(t.hour_num as u32, t.total);
            }
            _ => {}
        })
        .for_each(drop);

    hours_range.iter().for_each(|h| {
        daily_states
            .created
            .push(created_map.remove(h).unwrap_or(0));
        daily_states
            .timeout
            .push(timeout_map.remove(h).unwrap_or(0));
        daily_states
            .finished
            .push(finished_map.remove(h).unwrap_or(0));
        daily_states
            .abnormal
            .push(abnormal_map.remove(h).unwrap_or(0));
        daily_states
            .canceled
            .push(canceled_map.remove(h).unwrap_or(0));

        daily_states.hours_range.push(*h);
    });

    daily_states
}
