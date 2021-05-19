use super::prelude::*;

pub(crate) fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(show_one_day_tasks_state);
}

#[get("/api/tasks_state/one_day")]
async fn show_one_day_tasks_state(pool: ShareData<db::ConnectionPool>) -> HttpResponse {
    use db::schema::task_log;

    if let Ok(conn) = pool.get() {
        return HttpResponse::Ok().json(
            Into::<UnifiedResponseMessages<model::PaginateTask>>::into(
                web::block::<_, _, diesel::result::Error>(move || {
                    let query_builder = model::TaskQueryBuilder::query_all_columns();

                    let now = NaiveDateTime::from_timestamp(get_timestamp() as i64, 0);
                    let past_day = now - ChronoDuration::days(1);

                    // That's is ok.
                    // task_log::table
                    //     .select(
                    //         diesel::dsl::count(task_log::id),
                    //     )
                    //     .filter(task_log::created_time.between(past_day, now));

                    // Can't package in a tuple.
                    // let task_state: Vec<model::TaskState> = task_log::table
                    //     .select((
                    //         task_log::created_time,
                    //         task_log::status,
                    //         // dsl::count(task_log::id),
                    //     ))
                    //     .filter(task_log::created_time.between(past_day, now))
                    //     .load(&conn)?;

                    todo!();
                })
                .await,
            ),
        );
    }

    HttpResponse::Ok().json(UnifiedResponseMessages::<model::PaginateTask>::error())
}
