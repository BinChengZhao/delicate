use super::prelude::*;

pub(crate) fn config_route(route: Route) -> Route {
    route
        .at("/api/task/create", post(create_task))
        .at("/api/task/list", post(show_tasks))
        // FIXME:
        // .at("/api/task/update", post(update_task))
        // .at("/api/task/run", post(run_task))
        // .at("/api/task/suspend", post(suspend_task))
        // .at("/api/task/advance", post(advance_task))
        .at("/api/task/delete", post(delete_task))
}

#[handler]

async fn create_task(
    _req: &Request,
    Json(model::NewTaskBody { task, binding_ids }): Json<model::NewTaskBody>,
    pool: Data<&db::ConnectionPool>,
) -> impl IntoResponse {
    use db::schema::{task, task_bind};

    if let Ok(conn) = pool.get() {
        // // FIXME:
        // let operation_log_pair_option =
        //     generate_operation_task_addtion_log(&req.get_session(), &task).ok();
        // send_option_operation_log_pair(operation_log_pair_option).await;

        let f_result = spawn_blocking::<_, Result<_, diesel::result::Error>>(move || {
            conn.transaction::<_, _, _>(|| {
                diesel::insert_into(task::table)
                    .values(&task)
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
        })
        .await;

        let count = f_result
            .map(Into::<UnifiedResponseMessages<usize>>::into)
            .unwrap_or_else(|e| {
                UnifiedResponseMessages::<usize>::error().customized_error_msg(e.to_string())
            });
        return Json(count);
    }

    Json(UnifiedResponseMessages::<usize>::error())
}

#[handler]

async fn show_tasks(
    Json(query_params): Json<model::QueryParamsTask>,
    pool: Data<&db::ConnectionPool>,
) -> impl IntoResponse {
    use db::schema::task_bind;

    if let Ok(conn) = pool.get() {
        let f_result = spawn_blocking::<_, Result<_, diesel::result::Error>>(move || {
            let query_builder = model::TaskQueryBuilder::query_all_columns();

            let mut tasks: HashMap<i64, model::FrontEndTask> = query_params
                .clone()
                .query_filter(query_builder)
                .paginate(query_params.page)
                .set_per_page(query_params.per_page)
                .load::<model::Task>(&conn)?
                .into_iter()
                .map(|t| (t.id, t.into()))
                .collect();

            let tasks_ids: Vec<i64> = tasks.iter().map(|(id, _)| *id).collect();

            let tasks_bind_pairs = task_bind::table
                .select((task_bind::task_id, task_bind::bind_id))
                .filter(task_bind::task_id.eq_any(&tasks_ids[..]))
                .load::<(i64, i64)>(&conn)?;

            tasks_bind_pairs.into_iter().for_each(|(task_id, bind_id)| {
                if let Some(task) = tasks.get_mut(&task_id) {
                    task.binding_ids.push(bind_id);
                }
            });

            let per_page = query_params.per_page;
            let count_builder = model::TaskQueryBuilder::query_count();
            let count = query_params
                .query_filter(count_builder)
                .get_result::<i64>(&conn)?;

            let mut front_end_task = tasks
                .into_iter()
                .map(|(_, t)| t)
                .collect::<Vec<model::FrontEndTask>>();

            front_end_task.sort_by(|a, b| a.id.cmp(&b.id));
            Ok(PaginateData::<model::FrontEndTask>::default()
                .set_data_source(front_end_task)
                .set_page_size(per_page)
                .set_total(count)
                .set_state_desc::<state::task::State>())
        })
        .await;

        let page = f_result
            .map(|page_result| {
                Into::<UnifiedResponseMessages<PaginateData<model::FrontEndTask>>>::into(
                    page_result,
                )
            })
            .unwrap_or_else(|e| {
                UnifiedResponseMessages::<PaginateData<model::FrontEndTask>>::error()
                    .customized_error_msg(e.to_string())
            });
        return Json(page);
    }

    Json(UnifiedResponseMessages::<PaginateData<model::FrontEndTask>>::error())
}

// FIXME:

// #[handler]
// async fn update_task(
//     req: &Request,
//     Json(update_task_body): Json<model::UpdateTaskBody>,
//     pool: Data<&db::ConnectionPool>,
// ) -> impl IntoResponse {
//     let respose: UnifiedResponseMessages<()> = pre_update_task(req, update_task_body, pool)
//         .instrument(span!(Level::INFO, "update-task"))
//         .await
//         .into();
//     Json(respose)
// }

pub async fn pre_update_task(
    _req: &Request,
    model::UpdateTaskBody { task, binding_ids }: model::UpdateTaskBody,
    pool: Data<&db::ConnectionPool>,
) -> Result<(), CommonError> {
    let task_id = task.id;
    let conn = pool.get()?;
    // FIXME:
    // let operation_log_pair_option =
    //     generate_operation_task_modify_log(&req.get_session(), &task).ok();
    // send_option_operation_log_pair(operation_log_pair_option).await;

    let task_binds_pair = pre_update_task_row(conn, task, binding_ids).await?;

    let conn = pool.get()?;
    pre_update_task_sevice(conn, task_id, task_binds_pair).await?;

    Ok(())
}

pub async fn pre_update_task_row(
    conn: db::PoolConnection,
    task: model::UpdateTask,
    binding_ids: Vec<i64>,
) -> Result<
    (
        Vec<model::BindProcessor>,
        Vec<model::BindProcessor>,
        Vec<model::BindProcessor>,
    ),
    CommonError,
> {
    use db::schema::{executor_processor, executor_processor_bind, task_bind};
    use model::BindProcessor;

    let task_binds_pair = spawn_blocking::<_, Result<_, diesel::result::Error>>(move || {
        conn.transaction(|| {
            let task_id = task.id;
            let update_effect_row = diesel::update(&task).set(&task).execute(&conn)?;

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
                .iter()
                .filter(|b| removed_task_binds_map.get(&b.bind_id).is_some())
                .cloned()
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

            let reserved_bind_processors: Vec<model::BindProcessor> = if update_effect_row != 0 {
                let reserved_task_binds_set: HashSet<i64> = original_task_binds
                    .intersection(&current_task_binds)
                    .copied()
                    .into_iter()
                    .collect();

                original_bind_processors
                    .into_iter()
                    .filter(|b| reserved_task_binds_set.contains(&b.bind_id))
                    .collect()
            } else {
                Vec::new()
            };

            Ok((
                removed_bind_processors,
                append_bind_processors,
                reserved_bind_processors,
            ))
        })
    })
    .await??;

    Ok(task_binds_pair)
}

pub async fn pre_update_task_sevice(
    conn: db::PoolConnection,
    task_id: i64,
    (_removed_bind_processors, _append_bind_processors, _reserved_bind_processors): (
        Vec<model::BindProcessor>,
        Vec<model::BindProcessor>,
        Vec<model::BindProcessor>,
    ),
) -> Result<(), CommonError> {
    use db::schema::task;
    use delicate_utils_task::TaskPackage;

    let (task_package, _status) = task::table
        .select((
            (
                task::id,
                task::command,
                task::frequency,
                task::cron_expression,
                task::timeout,
                task::maximum_parallel_runnable_num,
            ),
            task::status,
        ))
        .filter(task::id.eq(task_id))
        .first::<(TaskPackage, i16)>(&conn)?;

    let _task_id = task_package.id;

    // FIXME:
    todo!();
    // if status == state::task::State::Enabled as i16 {
    //     let remove_tasks_future: JoinAll<_> = removed_bind_processors
    //         .into_iter()
    //         .filter_map(|processor| {
    //             let executor_host =
    //                 "http://".to_string() + (processor.host.deref()) + "/api/task/remove";

    //             let message = delicate_utils_task::TaskUnit::default()
    //                 .set_task_id(task_id)
    //                 .set_time(get_timestamp());

    //             info!("Remove task{} at:{}", &task_package, &executor_host);
    //             message
    //                 .sign(Some(&processor.token))
    //                 .map(|s| (s, executor_host))
    //                 .ok()
    //         })
    //         .map(|(signed_task_unit, executor_host)| {
    //             RequestClient::default()
    //                 .post(executor_host)
    //                 .send_json(&signed_task_unit)
    //         })
    //         .collect::<Vec<_>>()
    //         .into_iter()
    //         .collect();

    //     let append_tasks_future: JoinAll<_> = append_bind_processors
    //         .into_iter()
    //         .filter_map(|processor| {
    //             let executor_host =
    //                 "http://".to_string() + (processor.host.deref()) + "/api/task/create";

    //             info!("Create task{} at:{}", &task_package, &executor_host);
    //             task_package
    //                 .clone()
    //                 .sign(Some(&processor.token))
    //                 .map(|s| (s, executor_host))
    //                 .ok()
    //         })
    //         .map(|(signed_task_package, executor_host)| {
    //             RequestClient::default()
    //                 .post(executor_host)
    //                 .send_json(&signed_task_package)
    //         })
    //         .collect::<Vec<_>>()
    //         .into_iter()
    //         .collect();

    //     let update_tasks_future: JoinAll<_> = reserved_bind_processors
    //         .into_iter()
    //         .filter_map(|processor| {
    //             let executor_host =
    //                 "http://".to_string() + (processor.host.deref()) + "/api/task/update";

    //             info!("Update task {} at:{}", &task_package, &executor_host);
    //             task_package
    //                 .clone()
    //                 .sign(Some(&processor.token))
    //                 .map(|s| (s, executor_host))
    //                 .ok()
    //         })
    //         .map(|(signed_task_package, executor_host)| {
    //             RequestClient::default()
    //                 .post(executor_host)
    //                 .send_json(&signed_task_package)
    //         })
    //         .collect::<Vec<_>>()
    //         .into_iter()
    //         .collect();

    //     join3(
    //         handle_response::<UnifiedResponseMessages<()>>(remove_tasks_future),
    //         handle_response::<UnifiedResponseMessages<()>>(append_tasks_future),
    //         handle_response::<UnifiedResponseMessages<()>>(update_tasks_future),
    //     )
    //     .await;
    // }

    // Ok(())
}

#[handler]

async fn delete_task(
    _req: &Request,
    Json(model::TaskId { task_id }): Json<model::TaskId>,
    pool: Data<&db::ConnectionPool>,
) -> impl IntoResponse {
    use db::schema::{task, task_bind};

    // FIXME:

    // let operation_log_pair_option = generate_operation_task_delete_log(
    //     &req.get_session(),
    //     &CommonTableRecord::default().set_id(task_id),
    // )
    // .ok();
    // send_option_operation_log_pair(operation_log_pair_option).await;

    // delete
    if let Ok(conn) = pool.get() {
        let f_result = spawn_blocking::<_, Result<_, diesel::result::Error>>(move || {
            diesel::delete(task::table.find(task_id)).execute(&conn)?;
            diesel::delete(task_bind::table.filter(task_bind::task_id.eq(task_id)))
                .execute(&conn)?;
            Ok(())
        })
        .await;

        let resp = f_result
            .map(Into::<UnifiedResponseMessages<()>>::into)
            .unwrap_or_else(|e| {
                UnifiedResponseMessages::<()>::error().customized_error_msg(e.to_string())
            });
        return Json(resp);
    }

    Json(UnifiedResponseMessages::<()>::error())
}

// FIXME:

// #[handler]
// async fn run_task(
//     req: &Request,
//     Json(model::TaskId { task_id }): Json<model::TaskId>,
//     pool: Data<&db::ConnectionPool>,
// ) -> impl IntoResponse {
//     let result: UnifiedResponseMessages<()> = Into::into(
//         pre_run_task(req, task_id, pool)
//             .instrument(span!(Level::INFO, "run-task"))
//             .await,
//     );

//     Json(result)
// }

// FIXME:

// #[handler]

// async fn suspend_task(
//     req: &Request,
//     Json(model::TaskId { task_id }): Json<model::TaskId>,
//     pool: Data<&Arc<db::ConnectionPool>>,
// ) -> impl IntoResponse {
//     let result: UnifiedResponseMessages<()> = Into::into(
//         pre_operate_task(req, pool.clone(), (task_id, "/api/task/remove", "Suspend"))
//             .instrument(span!(Level::INFO, "Suspend", task_id))
//             .await,
//     );

//     Json(result)
// }

// FIXME:

// #[handler]
// async fn advance_task(
//     req: &Request,
//     Json(model::TaskId { task_id }): Json<model::TaskId>,
//     pool: Data<&Arc<db::ConnectionPool>>,
// ) -> impl IntoResponse {
//     let result: UnifiedResponseMessages<()> = Into::into(
//         pre_operate_task(req, pool, (task_id, "/api/task/advance", "Advance"))
//             .instrument(span!(Level::INFO, "Advance", task_id))
//             .await,
//     );

//     Json(result)
// }

async fn pre_run_task(
    _req: &Request,
    task_id: i64,
    pool: Data<&db::ConnectionPool>,
) -> Result<(), CommonError> {
    use db::schema::executor_processor::dsl::{host, token};
    use db::schema::task::dsl::*;
    use db::schema::{executor_processor, executor_processor_bind, task, task_bind};

    use state::task::State;

    // FIXME:

    // let operation_log_pair_option = generate_operation_task_modify_log(
    //     &req.get_session(),
    //     &CommonTableRecord::default()
    //         .set_id(task_id)
    //         .set_description("Run task"),
    // )
    // .ok();
    // send_option_operation_log_pair(operation_log_pair_option).await;

    let conn = pool.get()?;

    // Many machine.
    let _task_packages: Vec<(delicate_utils_task::TaskPackage, (String, String))> =
        spawn_blocking::<_, Result<_, diesel::result::Error>>(move || {
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
                        maximum_parallel_runnable_num,
                    ),
                    (host, token),
                ))
                .filter(task_bind::task_id.eq(task_id))
                .load::<(delicate_utils_task::TaskPackage, (String, String))>(&conn)
        })
        .await??;

    // FIXME:
    todo!();
    // let request_all: JoinAll<_> = task_packages
    //     .into_iter()
    //     .filter_map(|(task_package, (executor_host_str, executor_token))| {
    //         let executor_host =
    //             "http://".to_string() + (executor_host_str.deref()) + "/api/task/create";
    //         info!("Run task{} at:{}", &task_package, &executor_host);
    //         task_package
    //             .sign(Some(&executor_token))
    //             .map(|s| (s, executor_host))
    //             .ok()
    //     })
    //     .map(|(signed_task_package, executor_host)| {
    //         RequestClient::default()
    //             .post(executor_host)
    //             .send_json(&signed_task_package)
    //     })
    //     .collect::<Vec<_>>()
    //     .into_iter()
    //     .collect();

    // handle_response::<UnifiedResponseMessages<()>>(request_all).await;

    // Ok(())
}

// FIXME:

// async fn pre_operate_task(
//     req: &Request,
//     pool: Arc<db::ConnectionPool>,
//     (task_id, url, action): (i64, &str, &'static str),
// ) {
//     use db::schema::executor_processor::dsl::{host, token};
//     use db::schema::{executor_processor, executor_processor_bind, task, task_bind};
//     use state::task::State;

// let conn = pool.get()?;

// let operation_log_pair_option = generate_operation_task_modify_log(
//     &req.get_session(),
//     &CommonTableRecord::default()
//         .set_id(task_id)
//         .set_description(action),
// )
// .ok();
// send_option_operation_log_pair(operation_log_pair_option).await;

// Many machine.
// let executor_packages: IntoIter<(String, String)> = spawn_blocking::<_,Result<_, diesel::result::Error>>(move || {
//     // TODO: Optimize.
//     if action.eq("Suspend") {
//         diesel::update(task::table.find(task_id))
//             .set(task::status.eq(State::NotEnabled as i16))
//             .execute(&conn)?;
//     }

//     task_bind::table
//         .inner_join(executor_processor_bind::table.inner_join(executor_processor::table))
//         .inner_join(task::table)
//         .select((host, token))
//         .filter(task_bind::task_id.eq(task_id))
//         .load::<(String, String)>(&conn)
// })
// .await?
// .into_iter();

// let request_all: JoinAll<SendClientRequest> = executor_packages
//     .filter_map(|(executor_host, executor_token)| {
//         let message = delicate_utils_task::TaskUnit::default()
//             .set_task_id(task_id)
//             .set_time(get_timestamp());

//         let executor_host = "http://".to_string() + (executor_host.deref()) + url;

//         info!("{} task{} at:{}", action, message, &executor_host);
//         message
//             .sign(Some(&executor_token))
//             .map(|s| (s, executor_host))
//             .ok()
//     })
//     .map(|(signed_task_unit, executor_host)| {
//         RequestClient::builder()
//             .timeout(Duration::from_secs(15))
//             .finish()
//             .post(executor_host)
//             .send_json(&signed_task_unit)
//     })
//     .collect::<Vec<SendClientRequest>>()
//     .into_iter()
//     .collect();

// handle_response::<UnifiedResponseMessages<()>>(request_all).await;
// Ok(())

// todo!();
// }
