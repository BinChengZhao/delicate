use super::prelude::*;

pub(crate) fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(show_one_day_tasks_state);
}

#[get("/api/tasks_state/one_day")]
async fn show_one_day_tasks_state(pool: ShareData<db::ConnectionPool>) -> HttpResponse {
    use db::schema::task_log;
    use state::task_log::State;

    if let Ok(conn) = pool.get() {
        let daily_state_result = web::block::<_, _, diesel::result::Error>(move || {
            let now = NaiveDateTime::from_timestamp(get_timestamp() as i64, 0);
            let past_day = now - ChronoDuration::days(1);
            let hours_range: Vec<u32> =
                ((past_day.hour() + 1)..).take(24).map(|n| n % 24).collect();


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
        return HttpResponse::Ok().json(Into::<UnifiedResponseMessages<model::DailyState>>::into(
            daily_state_result,
        ));
    }

    HttpResponse::Ok().json(UnifiedResponseMessages::<model::DailyState>::error())
}

pub(crate) fn pre_show_one_day_tasks_state(
    hours_range: Vec<u32>,
    create_count: Vec<model::TaskState>,
    end_states_count: Vec<model::TaskState>,
) -> model::DailyState {
    use state::task_log::State;
    use std::collections::HashMap;

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
