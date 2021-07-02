use super::prelude::*;

pub(crate) fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(show_tasks)
        .service(create_task)
        .service(update_task)
        .service(run_task)
        .service(suspend_task)
        .service(advance_task)
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
            Into::<UnifiedResponseMessages<PaginateData<model::Task>>>::into(
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

                    Ok(PaginateData::<model::Task>::default()
                        .set_data_source(tasks)
                        .set_page_size(per_page)
                        .set_total(count))
                })
                .await,
            ),
        );
    }

    HttpResponse::Ok().json(UnifiedResponseMessages::<PaginateData<model::Task>>::error())
}

#[post("/api/task/update")]
async fn update_task(
    web::Json(update_task_body): web::Json<model::UpdateTaskBody>,
    pool: ShareData<db::ConnectionPool>,
) -> HttpResponse {
    let respose: UnifiedResponseMessages<()> = pre_update_task(update_task_body, pool).await.into();
    HttpResponse::Ok().json(respose)
}

pub async fn pre_update_task(
    model::UpdateTaskBody { task, binding_ids }: model::UpdateTaskBody,
    pool: ShareData<db::ConnectionPool>,
) -> Result<(), CommonError> {
    let task_id = task.id;
    let conn = pool.get()?;
    let task_binds_pair = pre_update_task_row(conn, task, binding_ids).await?;

    let conn = pool.get()?;
    pre_update_task_sevice(conn, task_id, task_binds_pair).await?;

    Ok(())
}

pub async fn pre_update_task_row(
    conn: db::PoolConnection,
    task: model::UpdateTask,
    binding_ids: Vec<i64>,
) -> Result<(Vec<model::BindProcessor>, Vec<model::BindProcessor>), CommonError> {
    use db::schema::{executor_processor, executor_processor_bind, task_bind};
    use model::BindProcessor;
    use std::collections::{HashMap, HashSet};

    let task_binds_pair = web::block::<_, _, diesel::result::Error>(move || {
        conn.transaction(|| {
            let task_id = task.id;
            diesel::update(&task).set(&task).execute(&conn)?;

            let original_bind_processors: Vec<BindProcessor> = task_bind::table
                .inner_join(executor_processor_bind::table.inner_join(executor_processor::table))
                .select((
                    task_bind::bind_id,
                    executor_processor::host,
                    executor_processor::token,
                ))
                .filter(task_bind::task_id.eq(task_id))
                .load(&conn)?;

            // Contrast with binding updates.
            let original_task_binds: HashSet<i64> =
                original_bind_processors.iter().map(|b| b.bind_id).collect();

            let current_task_binds: HashSet<i64> = binding_ids.into_iter().collect();

            let removed_task_binds_set = original_task_binds.difference(&current_task_binds);

            let removed_task_binds: Vec<model::NewTaskBind> = removed_task_binds_set
                .clone()
                .into_iter()
                .copied()
                .map(|bind_id| model::NewTaskBind { task_id, bind_id })
                .collect();

            for model::NewTaskBind { task_id, bind_id } in removed_task_binds.iter() {
                diesel::delete(
                    task_bind::table
                        .filter(task_bind::task_id.eq(task_id))
                        .filter(task_bind::bind_id.eq(bind_id)),
                )
                .execute(&conn)?;
            }

            let append_task_binds: Vec<model::NewTaskBind> = current_task_binds
                .difference(&original_task_binds)
                .into_iter()
                .copied()
                .map(|bind_id| model::NewTaskBind { task_id, bind_id })
                .collect();

            diesel::insert_into(task_bind::table)
                .values(&append_task_binds[..])
                .execute(&conn)?;

            let removed_task_binds_map: HashMap<i64, ()> =
                removed_task_binds_set.map(|b| (*b, ())).collect();

            let removed_bind_processors: Vec<BindProcessor> = original_bind_processors
                .into_iter()
                .filter(|b| removed_task_binds_map.get(&b.bind_id).is_some())
                .collect();

            let append_binds: Vec<i64> = append_task_binds.iter().map(|b| b.bind_id).collect();

            let append_bind_processors: Vec<BindProcessor> = executor_processor_bind::table
                .inner_join(executor_processor::table)
                .select((
                    executor_processor_bind::id,
                    executor_processor::host,
                    executor_processor::token,
                ))
                .filter(executor_processor_bind::id.eq_any(&append_binds))
                .load(&conn)?;
            Ok((removed_bind_processors, append_bind_processors))
        })
    })
    .await?;

    Ok(task_binds_pair)
}

pub async fn pre_update_task_sevice(
    conn: db::PoolConnection,
    task_id: i64,
    (removed_bind_processors, append_bind_processors): (
        Vec<model::BindProcessor>,
        Vec<model::BindProcessor>,
    ),
) -> Result<(), CommonError> {
    use db::schema::task;
    use delicate_utils_task::TaskPackage;

    let (task_package, status) = task::table
        .select((
            (
                task::id,
                task::command,
                task::frequency,
                task::cron_expression,
                task::timeout,
                task::maximun_parallel_runnable_num,
            ),
            task::status,
        ))
        .filter(task::id.eq(task_id))
        .first::<(TaskPackage, i16)>(&conn)?;

    let task_id = task_package.id;

    if status == state::task::State::Enabled as i16 {
        let remove_tasks_future: JoinAll<_> = removed_bind_processors
            .into_iter()
            .filter_map(|processor| {
                let executor_host = "http://".to_string() + &processor.host + "/api/task/remove";

                let message = delicate_utils_task::TaskUnit::default()
                    .set_task_id(task_id)
                    .set_time(get_timestamp());

                info!("Remove task{} at:{}", &task_package, &executor_host);
                message
                    .sign(Some(&processor.token))
                    .map(|s| (s, executor_host))
                    .ok()
            })
            .map(|(signed_task_package, executor_host)| {
                RequestClient::default()
                    .post(executor_host)
                    .send_json(&signed_task_package)
            })
            .collect::<Vec<_>>()
            .into_iter()
            .collect();

        let append_tasks_future: JoinAll<_> = append_bind_processors
            .into_iter()
            .filter_map(|processor| {
                let executor_host = "http://".to_string() + &processor.host + "/api/task/create";

                info!("Remove task{} at:{}", &task_package, &executor_host);
                task_package
                    .clone()
                    .sign(Some(&processor.token))
                    .map(|s| (s, executor_host))
                    .ok()
            })
            .map(|(signed_task_package, executor_host)| {
                RequestClient::default()
                    .post(executor_host)
                    .send_json(&signed_task_package)
            })
            .collect::<Vec<_>>()
            .into_iter()
            .collect();

        join(remove_tasks_future, append_tasks_future).await;
    }

    Ok(())
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
    let result: UnifiedResponseMessages<()> =
        Into::into(pre_operate_task(pool.clone(), (task_id, "/api/task/remove", "Suspend")).await);

    HttpResponse::Ok().json(result)
}

#[post("/api/task/advance")]
async fn advance_task(
    web::Json(model::TaskId { task_id }): web::Json<model::TaskId>,
    pool: ShareData<db::ConnectionPool>,
) -> HttpResponse {
    let result: UnifiedResponseMessages<()> =
        Into::into(pre_operate_task(pool, (task_id, "/api/task/advance", "Advance")).await);

    HttpResponse::Ok().json(result)
}

async fn pre_run_task(
    task_id: i64,
    pool: ShareData<db::ConnectionPool>,
) -> Result<(), CommonError> {
    use db::schema::executor_processor::dsl::{host, token};
    use db::schema::task::dsl::*;
    use db::schema::{executor_processor, executor_processor_bind, task, task_bind};

    use state::task::State;

    let conn = pool.get()?;

    // Many machine.
    let task_packages: Vec<(delicate_utils_task::TaskPackage, (String, String))> =
        web::block(move || {
            diesel::update(task.find(task_id))
                .set(task::status.eq(State::Enabled as i16))
                .execute(&conn)?;

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
                    ),
                    (host, token),
                ))
                .filter(task_bind::task_id.eq(task_id))
                .load::<(delicate_utils_task::TaskPackage, (String, String))>(&conn)
        })
        .await?;

    let request_all: JoinAll<_> = task_packages
        .into_iter()
        .filter_map(|(task_package, (executor_host_str, executor_token))| {
            let executor_host = "http://".to_string() + &executor_host_str + "/api/task/create";
            info!("Run task{} at:{}", &task_package, &executor_host);
            task_package
                .sign(Some(&executor_token))
                .map(|s| (s, executor_host))
                .ok()
        })
        .map(|(signed_task_package, executor_host)| {
            RequestClient::default()
                .post(executor_host)
                .send_json(&signed_task_package)
        })
        .collect::<Vec<_>>()
        .into_iter()
        .collect();

    request_all.await;

    Ok(())
}

async fn pre_operate_task(
    pool: ShareData<db::ConnectionPool>,
    (task_id, url, action): (i64, &str, &'static str),
) -> Result<(), CommonError> {
    use db::schema::executor_processor::dsl::{host, token};
    use db::schema::{executor_processor, executor_processor_bind, task, task_bind};
    use state::task::State;

    let conn = pool.get()?;

    // Many machine.
    let executor_packages: IntoIter<(String, String)> = web::block(move || {
        // TODO: Optimize.
        if action.eq("Suspend") {
            diesel::update(task::table.find(task_id))
                .set(task::status.eq(State::NotEnabled as i16))
                .execute(&conn)?;
        }

        task_bind::table
            .inner_join(executor_processor_bind::table.inner_join(executor_processor::table))
            .inner_join(task::table)
            .select((host, token))
            .filter(task_bind::task_id.eq(task_id))
            .load::<(String, String)>(&conn)
    })
    .await?
    .into_iter();

    let request_all: JoinAll<_> = executor_packages
        .filter_map(|(executor_host, executor_token)| {
            let message = delicate_utils_task::TaskUnit::default()
                .set_task_id(task_id)
                .set_time(get_timestamp());

            let executor_host = "http://".to_string() + &executor_host + url;

            info!("{} task{} at:{}", action, message, &executor_host);
            message
                .sign(Some(&executor_token))
                .map(|s| (s, executor_host))
                .ok()
        })
        .map(|(signed_task_unit, executor_host)| {
            RequestClient::default()
                .post(executor_host)
                .send_json(&signed_task_unit)
        })
        .collect::<Vec<_>>()
        .into_iter()
        .collect();
    request_all.await;

    Ok(())
}
