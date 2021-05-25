use super::prelude::*;

pub(crate) fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(show_tasks)
        .service(create_task)
        .service(update_task)
        .service(delete_task);
}

#[post("/api/task/create")]
async fn create_task(
    web::Json(model::NewTaskBody {
        new_task,
        binding_ids,
    }): web::Json<model::NewTaskBody>,
    pool: ShareData<db::ConnectionPool>,
) -> HttpResponse {
    use db::schema::{task, task_bind};

    if let Ok(conn) = pool.get() {
        return HttpResponse::Ok().json(Into::<UnifiedResponseMessages<usize>>::into(
            web::block::<_, _, diesel::result::Error>(move || {
                diesel::insert_into(task::table)
                    .values(&new_task)
                    .execute(&conn)?;
                let task_id = diesel::select(db::last_insert_id).get_result::<u64>(&conn)? as i64;

                let new_task_binds: Vec<model::NewTaskBind> = binding_ids
                    .into_iter()
                    .map(|bind_id| model::NewTaskBind { task_id, bind_id })
                    .collect();

                diesel::insert_into(task_bind::table)
                    .values(&new_task_binds)
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
    web::Json(model::UpdateTaskBody { task, binding_ids }): web::Json<model::UpdateTaskBody>,
    pool: ShareData<db::ConnectionPool>,
) -> HttpResponse {
    use db::schema::task_bind;
    use std::collections::HashSet;

    if let Ok(conn) = pool.get() {
        return HttpResponse::Ok().json(Into::<UnifiedResponseMessages<()>>::into(
            web::block::<_, _, diesel::result::Error>(move || {
                conn.transaction(|| {
                    diesel::update(&task).set(&task).execute(&conn)?;

                    // Contrast with binding updates.
                    let original_task_binds: HashSet<i64> = task_bind::table
                        .select(task_bind::bind_id)
                        .filter(task_bind::task_id.eq(task.id))
                        .load(&conn)?
                        .into_iter()
                        .collect();

                    let current_task_binds: HashSet<i64> = binding_ids.into_iter().collect();

                    let task_id = task.id;

                    let removed_task_binds: Vec<model::NewTaskBind> = original_task_binds
                        .difference(&current_task_binds)
                        .into_iter()
                        .map(|b| *b)
                        .map(|bind_id| model::NewTaskBind { bind_id, task_id })
                        .collect();

                    let append_task_binds: Vec<model::NewTaskBind> = current_task_binds
                        .difference(&original_task_binds)
                        .into_iter()
                        .map(|b| *b)
                        .map(|bind_id| model::NewTaskBind { bind_id, task_id })
                        .collect();

                    for model::NewTaskBind { task_id, bind_id } in removed_task_binds {
                        diesel::delete(
                            task_bind::table
                                .filter(task_bind::task_id.eq(task_id))
                                .filter(task_bind::bind_id.eq(bind_id)),
                        )
                        .execute(&conn)?;
                    }

                    diesel::insert_into(task_bind::table)
                        .values(&append_task_binds[..])
                        .execute(&conn)?;
                    Ok(())
                })
            })
            .await,
        ));
    }

    HttpResponse::Ok().json(UnifiedResponseMessages::<()>::error())
}
#[post("/api/task/delete")]
async fn delete_task(
    web::Json(model::TaskId { task_id }): web::Json<model::TaskId>,
    pool: ShareData<db::ConnectionPool>,
) -> HttpResponse {
    use db::schema::{task, task_bind};

    if let Ok(conn) = pool.get() {
        return HttpResponse::Ok().json(Into::<UnifiedResponseMessages<()>>::into(
            web::block::<_, _, diesel::result::Error>(move || {
                diesel::delete(task::table.find(task_id)).execute(&conn)?;
                diesel::delete(task_bind::table.filter(task_bind::task_id.eq(task_id)))
                    .execute(&conn)?;
                Ok(())
            })
            .await,
        ));
    }

    HttpResponse::Ok().json(UnifiedResponseMessages::<()>::error())
}

#[post("/api/task/run")]
async fn run_task(
    web::Path(task_id): web::Path<i64>,
    pool: ShareData<db::ConnectionPool>,
) -> HttpResponse {
    use actix_web::client::Client;
    use db::schema::executor_processor::dsl::*;
    use db::schema::{executor_processor, executor_processor_bind, task_bind};

    if let Ok(conn) = pool.get() {
        let _executor_processor_result: Result<Vec<(String, i16)>, _> = web::block(move || {
            task_bind::table
                .inner_join(executor_processor_bind::table.inner_join(executor_processor::table))
                .select((host, port))
                .filter(task_bind::task_id.eq(task_id))
                .filter(status.eq(state::executor_processor::State::Health as i16))
                .load(&conn)
        })
        .await;

        // TODO: Send task.

        let mut _client = Client::default();
        todo!();
    }

    HttpResponse::Ok().json(UnifiedResponseMessages::<usize>::error())
}
