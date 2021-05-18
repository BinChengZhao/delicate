use super::prelude::*;

pub(crate) fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(show_tasks)
        .service(create_task)
        .service(update_task)
        .service(delete_task);
}

#[post("/api/task/create")]
async fn create_task(
    task: web::Json<model::NewTask>,
    pool: ShareData<db::ConnectionPool>,
) -> HttpResponse {
    use db::schema::task;

    if let Ok(conn) = pool.get() {
        return HttpResponse::Ok().json(Into::<UnifiedResponseMessages<usize>>::into(
            web::block(move || {
                diesel::insert_into(task::table)
                    .values(&*task)
                    .execute(&conn)
            })
            .await,
        ));
    }

    HttpResponse::Ok().json(UnifiedResponseMessages::<()>::error())
}

#[post("/api/task/list")]
async fn show_tasks(
    web::Json(query_params): web::Json<model::QueryParamsTask>,
    pool: ShareData<db::ConnectionPool>,
) -> HttpResponse {
    if let Ok(conn) = pool.get() {
        return HttpResponse::Ok().json(
            Into::<UnifiedResponseMessages<model::PaginateTask>>::into(
                web::block::<_, _, diesel::result::Error>(move || {
                    let query_builder = model::TaskQueryBuilder::query_all_columns();

                    let tasks = query_params
                        .clone()
                        .query_filter(query_builder)
                        .paginate(query_params.page)
                        .load::<model::Task>(&conn)?;

                    let per_page = query_params.per_page;
                    let count_builder = model::TaskQueryBuilder::query_count();
                    let count = query_params
                        .query_filter(count_builder)
                        .get_result::<i64>(&conn)?;

                    Ok(model::task::PaginateTask::default()
                        .set_tasks(tasks)
                        .set_per_page(per_page)
                        .set_total_page(count))
                })
                .await,
            ),
        );
    }

    HttpResponse::Ok().json(UnifiedResponseMessages::<model::PaginateTask>::error())
}

#[post("/api/task/update")]
async fn update_task(
    web::Json(task_value): web::Json<model::NewTask>,
    pool: ShareData<db::ConnectionPool>,
) -> HttpResponse {
    if let Ok(conn) = pool.get() {
        return HttpResponse::Ok().json(Into::<UnifiedResponseMessages<usize>>::into(
            web::block(move || diesel::update(&task_value).set(&task_value).execute(&conn)).await,
        ));
    }

    HttpResponse::Ok().json(UnifiedResponseMessages::<usize>::error())
}
#[post("/api/task/delete")]
async fn delete_task(
    web::Path(task_id): web::Path<i64>,
    pool: ShareData<db::ConnectionPool>,
) -> HttpResponse {
    use db::schema::task::dsl::*;

    if let Ok(conn) = pool.get() {
        return HttpResponse::Ok().json(Into::<UnifiedResponseMessages<usize>>::into(
            web::block(move || diesel::delete(task.find(task_id)).execute(&conn)).await,
        ));
    }

    HttpResponse::Ok().json(UnifiedResponseMessages::<usize>::error())
}

#[post("/api/task_log/list")]
async fn show_task_logs(
    web::Json(query_params): web::Json<model::QueryParamsTaskLog>,
    pool: ShareData<db::ConnectionPool>,
) -> HttpResponse {
    if let Ok(conn) = pool.get() {
        return HttpResponse::Ok().json(
            Into::<UnifiedResponseMessages<model::PaginateTaskLogs>>::into(
                web::block::<_, _, diesel::result::Error>(move || {
                    let query_builder = model::TaskLogQueryBuilder::query_all_columns();

                    let task_logs = query_params
                        .clone()
                        .query_filter(query_builder)
                        .paginate(query_params.page)
                        .load::<model::TaskLog>(&conn)?;

                    let per_page = query_params.per_page;
                    let count_builder = model::TaskLogQueryBuilder::query_count();
                    let count = query_params
                        .query_filter(count_builder)
                        .get_result::<i64>(&conn)?;

                    Ok(model::task_log::PaginateTaskLogs::default()
                        .set_task_logs(task_logs)
                        .set_per_page(per_page)
                        .set_total_page(count))
                })
                .await,
            ),
        );
    }

    HttpResponse::Ok().json(UnifiedResponseMessages::<model::PaginateTask>::error())
}

// Expose api, not for log adding, but for event replies
// Depending on the event, scheduler records/updates different logs.
// Bulk operations are supported for log messages passed from delicate-executor.
#[post("/api/task_logs/event_trigger")]
async fn create_task_logs(
    web::Json(events_collection): web::Json<model::ExecutorEventCollection>,
    pool: ShareData<db::ConnectionPool>,
) -> HttpResponse {
    use db::common::types::EventType;
    use db::schema::task_log;

    if !events_collection.verify_signature("") {
        return HttpResponse::Ok().json(
            UnifiedResponseMessages::<usize>::error()
                .customized_error_msg(String::from("Signature verification failure.")),
        );
    }

    if let Ok(conn) = pool.get() {
        return HttpResponse::Ok().json(Into::<UnifiedResponseMessages<usize>>::into(
            web::block::<_, _, diesel::result::Error>(move || {
                conn.transaction(|| {
                    let mut effect_num = 0;
                    let model::ExecutorEventCollection { events, .. } = events_collection;
                    let mut new_task_logs: Vec<model::NewTaskLog> = Vec::new();
                    let mut supply_task_logs: Vec<model::SupplyTaskLog> = Vec::new();

                    events
                        .into_iter()
                        .for_each(|e| match Into::<EventType>::into(e.event_type) {
                            EventType::TaskPerform => new_task_logs.push(e.into()),
                            EventType::Unknown => {}
                            _ => supply_task_logs.push(e.into()),
                        });

                    effect_num += diesel::insert_into(task_log::table)
                        .values(&new_task_logs[..])
                        .execute(&conn)?;

                    for supply_task_log in supply_task_logs {
                        effect_num += diesel::update(&supply_task_log)
                            .set(&supply_task_log)
                            .execute(&conn)?;
                    }

                    Ok(effect_num)
                })
            })
            .await,
        ));
    }

    HttpResponse::Ok().json(UnifiedResponseMessages::<usize>::error())
}
