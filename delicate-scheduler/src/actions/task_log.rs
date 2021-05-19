use super::prelude::*;

pub(crate) fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(create_task_logs).service(show_task_logs);
}

// Depending on the event, scheduler records/updates different logs.
// Bulk operations are supported for log messages passed from delicate-executor.
#[post("/api/task_logs/event_trigger")]
async fn create_task_logs(
    web::Json(events_collection): web::Json<model::ExecutorEventCollection>,
    pool: ShareData<db::ConnectionPool>,
) -> HttpResponse {
    use db::common::types::EventType;

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

                    effect_num += batch_insert_task_logs(&conn, new_task_logs)?;
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

fn batch_insert_task_logs(
    conn: &db::PoolConnection,
    mut new_task_logs: Vec<model::NewTaskLog>,
) -> QueryResult<usize> {
    use db::schema::{task, task_log};
    use std::collections::HashMap;

    if !new_task_logs.is_empty() {
        let task_ids: Vec<i64> = new_task_logs.iter().map(|e| e.task_id).collect();

        let tasks: HashMap<i64, model::task::SupplyTask> =
            model::TaskQueryBuilder::query_supply_task_log()
                .filter(task::id.eq_any(&task_ids[..]))
                .load::<model::task::SupplyTask>(conn)?
                .into_iter()
                .map(|t| (t.id, t))
                .collect();

        new_task_logs.iter_mut().for_each(|t| {
            let task = &tasks[&t.id];
            t.name.clone_from(&task.name);
            t.description.clone_from(&task.description);
            t.command.clone_from(&task.command);
            t.frequency.clone_from(&task.frequency);
            t.cron_expression.clone_from(&task.cron_expression);
            t.tag.clone_from(&task.tag);
        });

        return diesel::insert_into(task_log::table)
            .values(&new_task_logs[..])
            .execute(conn);
    }

    Ok(0)
}
