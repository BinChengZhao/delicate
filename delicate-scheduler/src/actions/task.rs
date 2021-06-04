use super::prelude::*;

pub(crate) fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(show_tasks)
        .service(create_task)
        .service(update_task)
        .service(run_task)
        .service(suspend_task)
        .service(manual_trigger_task)
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
    web::Json(model::TaskId { task_id }): web::Json<model::TaskId>,
    pool: ShareData<db::ConnectionPool>,
) -> HttpResponse {
    let result: UnifiedResponseMessages<()> = Into::into(pre_run_task(task_id, pool).await);

    HttpResponse::Ok().json(result)
}

#[post("/api/task/suspend")]
async fn suspend_task(
    web::Json(model::TaskId { task_id }): web::Json<model::TaskId>,
    pool: ShareData<db::ConnectionPool>,
) -> HttpResponse {
    let result: UnifiedResponseMessages<()> = Into::into(pre_suspend_task(task_id, pool).await);

    HttpResponse::Ok().json(result)
}


#[post("/api/task/manual_trigger")]
async fn manual_trigger_task(
    web::Json(model::TaskId { task_id }): web::Json<model::TaskId>,
    pool: ShareData<db::ConnectionPool>,
) -> HttpResponse {
    use db::schema::executor_processor::dsl::*;
    use db::schema::{executor_processor, executor_processor_bind, task_bind};

    if let Ok(conn) = pool.get() {
        // Many machine.
        let _executor_processor_result: Result<Vec<(String, String)>, _> = web::block(move || {
            task_bind::table
                .inner_join(executor_processor_bind::table.inner_join(executor_processor::table))
                .select((host, token))
                .filter(task_bind::task_id.eq(task_id))
                .load(&conn)
        })
        .await;

        // TODO: manual_trigger task.

        let mut _client = RequestClient::default();
        todo!();
    }

    HttpResponse::Ok().json(UnifiedResponseMessages::<usize>::error())
}

async fn pre_run_task(
    task_id: i64,
    pool: ShareData<db::ConnectionPool>,
) -> Result<(), crate_error::CommonError> {
    use db::schema::executor_processor::dsl::{host, token};
    use db::schema::task::dsl::*;
    use db::schema::{executor_processor, executor_processor_bind, task, task_bind};

    let conn = pool.get()?;

    // Many machine.
    let task_packages = web::block(move || {
        task_bind::table
            .inner_join(executor_processor_bind::table.inner_join(executor_processor::table))
            .inner_join(task::table)
            .select((
                (
                    id,
                    command,
                    frequency,
                    cron_expression,
                    timeout,
                    maximun_parallel_runnable_num,
                    host,
                ),
                (token),
            ))
            .filter(task_bind::task_id.eq(task_id))
            .load::<(model::TaskPackage, String)>(&conn)
    })
    .await?
    .into_iter();

    let client = RequestClient::default();
    for (task_package, executor_token) in task_packages {
        let executor_host = task_package.host.clone() + "/run";
        info!("Run task{} at:{}", &task_package, &executor_host);
        let signed_task_package = task_package.sign(executor_token)?;

        client
            .post(executor_host)
            .send_json(&signed_task_package)
            .await
            .map_err(|e| error!("{}", e))
            .ok();
    }

    Ok(())
}

async fn pre_suspend_task(
    task_id: i64,
    pool: ShareData<db::ConnectionPool>,
) -> Result<(), crate_error::CommonError> {
    use db::schema::executor_processor::dsl::{host, token};
    use db::schema::{executor_processor, executor_processor_bind, task, task_bind};

    let conn = pool.get()?;

    // Many machine.
    let executor_packages = web::block(move || {
        task_bind::table
            .inner_join(executor_processor_bind::table.inner_join(executor_processor::table))
            .inner_join(task::table)
            .select((host, token))
            .filter(task_bind::task_id.eq(task_id))
            .load::<(String, String)>(&conn)
    })
    .await?
    .into_iter();

    let client = RequestClient::default();
    for (executor_host, executor_token) in executor_packages {
        let message = model::SuspendTaskRecord::default()
            .set_task_id(task_id)
            .set_time(get_timestamp());

        let executor_host = executor_host + "/remove";

        info!("Suspend task{} at:{}", message, &executor_host);
        let signed_task_package = message.sign(executor_token)?;

        client
            .post(executor_host)
            .send_json(&signed_task_package)
            .await
            .map_err(|e| error!("{}", e))
            .ok();
    }

    Ok(())
}
