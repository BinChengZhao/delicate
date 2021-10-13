use super::prelude::*;

pub(crate) fn config_route(route: Route) -> Route {
    route
        .at("/api/executor_group/list", post(show_executor_groups))
        .at(
            "/api/executor_group/detail",
            post(show_executor_group_detail),
        )
        .at("/api/executor_group/create", post(create_executor_group))
        .at("/api/executor_group/update", post(update_executor_group))
        .at("/api/executor_group/delete", post(delete_executor_group))
}

#[handler]
async fn create_executor_group(
    req: &Request,
    Json(executor_group): Json<model::NewExecutorGroup>,
    pool: Data<&db::ConnectionPool>,
) -> impl IntoResponse {
    use db::schema::executor_group;

    // FIXME:
    // let operation_log_pair_option =
    //     generate_operation_executor_group_addtion_log(&req.get_session(), &executor_group).ok();
    // send_option_operation_log_pair(operation_log_pair_option).await;

    if let Ok(conn) = pool.get() {
        return Json(Into::<UnifiedResponseMessages<u64>>::into(
            web::block(move || {
                diesel::insert_into(executor_group::table)
                    .values(&executor_group)
                    .execute(&conn)?;

                diesel::select(db::last_insert_id).get_result::<u64>(&conn)
            })
            .await,
        ));
    }

    Json(UnifiedResponseMessages::<u64>::error())
}

#[handler]
async fn show_executor_groups(
    Json(query_params): Json<model::QueryParamsExecutorGroup>,
    pool: Data<&db::ConnectionPool>,
) -> impl IntoResponse {
    if let Ok(conn) = pool.get() {
        return Json(Into::<
            UnifiedResponseMessages<PaginateData<model::ExecutorGroup>>,
        >::into(
            web::block::<_, _, diesel::result::Error>(move || {
                let query_builder = model::ExecutorGroupQueryBuilder::query_all_columns();

                let executor_groups = query_params
                    .clone()
                    .query_filter(query_builder)
                    .paginate(query_params.page)
                    .set_per_page(query_params.per_page)
                    .load::<model::ExecutorGroup>(&conn)?;

                let per_page = query_params.per_page;
                let count_builder = model::ExecutorGroupQueryBuilder::query_count();
                let count = query_params
                    .query_filter(count_builder)
                    .get_result::<i64>(&conn)?;

                Ok(PaginateData::<model::ExecutorGroup>::default()
                    .set_data_source(executor_groups)
                    .set_page_size(per_page)
                    .set_total(count))
            })
            .await,
        ));
    }

    Json(UnifiedResponseMessages::<PaginateData<model::ExecutorGroup>>::error())
}

#[handler]
async fn show_executor_group_detail(
    Json(model::ExecutorGroupId { executor_group_id }): Json<model::ExecutorGroupId>,
    pool: Data<&db::ConnectionPool>,
) -> impl IntoResponse {
    let executor_group_detail_result =
        pre_show_executor_group_detail(executor_group_id, pool).await;
    if let Ok(executor_group_detail) = executor_group_detail_result {
        return Json(
            UnifiedResponseMessages::<model::ExecutorGroupDetail>::success_with_data(
                executor_group_detail,
            ),
        );
    };

    // FIXME:
    todo!();
    // Json(
    //     UnifiedResponseMessages::<model::ExecutorGroupDetail>::error()
    //         .customized_error_msg(executor_group_detail_result.expect_err("").to_string()),
    // )
}

async fn pre_show_executor_group_detail(
    executor_group_id: i64,
    pool: Data<&db::ConnectionPool>,
) -> Result<model::ExecutorGroupDetail, CommonError> {
    use db::schema::{executor_group, executor_processor, executor_processor_bind};

    let conn = pool.get()?;
    let executor_group_detail: model::ExecutorGroupDetail =
        web::block::<_, _, diesel::result::Error>(move || {
            let executor_group_detail_inner = executor_group::table
                .select(executor_group::all_columns)
                .find(executor_group_id)
                .first::<model::ExecutorGroup>(&conn)?;

            let bindings = executor_processor_bind::table
                .inner_join(executor_processor::table)
                .filter(executor_processor_bind::group_id.eq(executor_group_id))
                .select((
                    executor_processor_bind::id,
                    executor_processor_bind::name,
                    executor_processor_bind::executor_id,
                    executor_processor_bind::weight,
                    executor_processor::name,
                    executor_processor::host,
                    executor_processor::machine_id,
                ))
                .load::<model::ExecutorGroupBinding>(&conn)?;

            Ok(model::ExecutorGroupDetail {
                inner: executor_group_detail_inner,
                bindings,
            })
        })
        .await?;

    Ok(executor_group_detail)
}

#[handler]
async fn update_executor_group(
    req: &Request,
    Json(executor_group): Json<model::UpdateExecutorGroup>,
    pool: Data<&db::ConnectionPool>,
) -> impl IntoResponse {
    // FIXME:

    // let operation_log_pair_option =
    //     generate_operation_executor_group_modify_log(&req.get_session(), &executor_group).ok();
    // send_option_operation_log_pair(operation_log_pair_option).await;

    if let Ok(conn) = pool.get() {
        return Json(Into::<UnifiedResponseMessages<usize>>::into(
            web::block(move || {
                diesel::update(&executor_group)
                    .set(&executor_group)
                    .execute(&conn)
            })
            .await,
        ));
    }

    Json(UnifiedResponseMessages::<usize>::error())
}

#[handler]
async fn delete_executor_group(
    req: &Request,
    Json(model::ExecutorGroupId { executor_group_id }): Json<model::ExecutorGroupId>,
    pool: Data<&db::ConnectionPool>,
) -> impl IntoResponse {
    use db::schema::executor_group::dsl::*;

    // FIXME:

    // let operation_log_pair_option = generate_operation_executor_group_delete_log(
    //     &req.get_session(),
    //     &CommonTableRecord::default().set_id(executor_group_id),
    // )
    // .ok();
    // send_option_operation_log_pair(operation_log_pair_option).await;

    if let Ok(conn) = pool.get() {
        return Json(Into::<UnifiedResponseMessages<usize>>::into(
            web::block(move || {
                // Cannot link to delete internal bindings, otherwise it will cause data misalignment.
                diesel::delete(executor_group.find(executor_group_id)).execute(&conn)
            })
            .await,
        ));
    }

    Json(UnifiedResponseMessages::<usize>::error())
}
