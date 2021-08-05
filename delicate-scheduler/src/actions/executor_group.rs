use super::prelude::*;

pub(crate) fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(show_executor_groups)
        .service(show_executor_group_detail)
        .service(create_executor_group)
        .service(update_executor_group)
        .service(delete_executor_group);
}

#[post("/api/executor_group/create")]
async fn create_executor_group(
    req: HttpRequest,
    web::Json(executor_group): web::Json<model::NewExecutorGroup>,
    pool: ShareData<db::ConnectionPool>,
) -> HttpResponse {
    use db::schema::executor_group;

    let operation_log_pair_option =
        generate_operation_executor_group_addtion_log(&req.get_session(), &executor_group).ok();

    if let Ok(conn) = pool.get() {
        return HttpResponse::Ok().json(Into::<UnifiedResponseMessages<u64>>::into(
            web::block(move || {
                diesel::insert_into(executor_group::table)
                    .values(&executor_group)
                    .execute(&conn)?;
                operation_log_pair_option
                    .map(|operation_log_pair| operate_log(&conn, operation_log_pair));

                diesel::select(db::last_insert_id).get_result::<u64>(&conn)
            })
            .await,
        ));
    }

    HttpResponse::Ok().json(UnifiedResponseMessages::<()>::error())
}

#[post("/api/executor_group/list")]
async fn show_executor_groups(
    web::Json(query_params): web::Json<model::QueryParamsExecutorGroup>,
    pool: ShareData<db::ConnectionPool>,
) -> HttpResponse {
    if let Ok(conn) = pool.get() {
        return HttpResponse::Ok().json(Into::<
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

    HttpResponse::Ok().json(UnifiedResponseMessages::<PaginateData<model::ExecutorGroup>>::error())
}

#[post("/api/executor_group/detail")]
async fn show_executor_group_detail(
    web::Json(model::ExecutorGroupId { executor_group_id }): web::Json<model::ExecutorGroupId>,
    pool: ShareData<db::ConnectionPool>,
) -> HttpResponse {
    let executor_group_detail_result =
        pre_show_executor_group_detail(executor_group_id, pool).await;
    if let Ok(executor_group_detail) = executor_group_detail_result {
        return HttpResponse::Ok().json(
            UnifiedResponseMessages::<model::ExecutorGroupDetail>::success_with_data(
                executor_group_detail,
            ),
        );
    };
    HttpResponse::Ok().json(
        UnifiedResponseMessages::<()>::error()
            .customized_error_msg(executor_group_detail_result.expect_err("").to_string()),
    )
}

async fn pre_show_executor_group_detail(
    executor_group_id: i64,
    pool: ShareData<db::ConnectionPool>,
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

#[post("/api/executor_group/update")]
async fn update_executor_group(
    req: HttpRequest,
    web::Json(executor_group): web::Json<model::UpdateExecutorGroup>,
    pool: ShareData<db::ConnectionPool>,
) -> HttpResponse {
    let operation_log_pair_option =
        generate_operation_executor_group_modify_log(&req.get_session(), &executor_group).ok();

    if let Ok(conn) = pool.get() {
        return HttpResponse::Ok().json(Into::<UnifiedResponseMessages<usize>>::into(
            web::block(move || {
                operation_log_pair_option
                    .map(|operation_log_pair| operate_log(&conn, operation_log_pair));

                diesel::update(&executor_group)
                    .set(&executor_group)
                    .execute(&conn)
            })
            .await,
        ));
    }

    HttpResponse::Ok().json(UnifiedResponseMessages::<usize>::error())
}
#[post("/api/executor_group/delete")]
async fn delete_executor_group(
    req: HttpRequest,
    web::Json(model::ExecutorGroupId { executor_group_id }): web::Json<model::ExecutorGroupId>,
    pool: ShareData<db::ConnectionPool>,
) -> HttpResponse {
    use db::schema::executor_group::dsl::*;

    let operation_log_pair_option = generate_operation_executor_group_delete_log(
        &req.get_session(),
        &CommonTableRecord::default().set_id(executor_group_id),
    )
    .ok();

    if let Ok(conn) = pool.get() {
        return HttpResponse::Ok().json(Into::<UnifiedResponseMessages<usize>>::into(
            web::block(move || {
                operation_log_pair_option
                    .map(|operation_log_pair| operate_log(&conn, operation_log_pair));

                // Cannot link to delete internal bindings, otherwise it will cause data misalignment.
                diesel::delete(executor_group.find(executor_group_id)).execute(&conn)
            })
            .await,
        ));
    }

    HttpResponse::Ok().json(UnifiedResponseMessages::<usize>::error())
}
