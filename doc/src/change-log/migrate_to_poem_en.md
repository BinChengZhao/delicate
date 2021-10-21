# Title: `delicate` Why migrate from `actix-web` to `poem`.


## What is delicate ?

[delicate](https://github.com/BinChengZhao/delicate) A lightweight, distributed task scheduling platform. 


<a href="">
    <img src="https://delicate-rs-1301941387.cos.ap-beijing.myqcloud.com/delicate-rs/delicate_logo.png"
         alt="delicate logo" title="delicate" height="125" width="125" align="right"/>
</a>


## Features
- **Friendly user interface:** [Front-end] Easy management of tasks and executors, monitoring of their status, support for manual maintenance of running tasks, etc.

- **Flexible operation:** Flexible task operation, support for limiting the maximum number of parallelism for a single node, time zone setting corresponding to cron expressions, scheduling modes (single, fixed number, constantly repeating), ability to manually trigger tasks at any time, manually terminate task instances, online view of task logs.

- **High Availability:** Delicate supports horizontal scaling. High availability and performance is easily achieved by deploying as many Delicate servers and executors as possible.

- **High Performance:** Lightweight and basic features speed up performance, and the basic resource overhead of `delicate' is about (less than 0.1% cpu usage, 10m of memory...)

- **Observability:** There are many meaningful statistics that are regularly presented in graphs.

- **Upgradability:** Dynamic upgrade of the system (upgrade is done by getting the latest source code and doing database migration...)

- **Reusability:** The actuator provides `restful-api` that allows user applications to maintain custom tasks.

- ** Permissions management:** Permissions management features based on casbin implementation, continuous optimization of the experience.


`delicate` architecture diagram:

! [architecture](. /architecture.svg)

! [topology](... /topology.svg) /topology.svg)


## Project rendering
<details
<summary>Please click</summary>

! [](... /_media/dashboard.jpg)
! [](... /_media/executor_create.jpg)
! [](... /_media/executor_list.jpg)
! [](... /_media/group_create.jpg)
! [](... /_media/group_inner_bind.jpg)
! [](... /_media/login_en.jpg)
! [](... /_media/task_edit.jpg)
! [](... /_media/task_list_operation.jpg)
! [](... /_media/task_log_kill.jpg)
! [](... /_media/task_log_logs_2.jpg)
! [](... /_media/task_log_logs.jpg)
! [](... /_media/user_list.jpg)

</details


### Technology stack

* Backend ( scheduler & executor ): Rust  

* Original main dependencies: (actix-web & diesel & delay-timer & serde & tracing)

* Current main dependencies: (poem & tokio & diesel & delay-timer & serde & tracing)

* Frontend: antd-admin (React)

* Ui: Ant Design

* Database: mysql , postgres (plan support)



### Why migrate to `poem`?

* While iterating with `actix-web`, I was limited in my ability to upgrade core dependencies and introduce new features because `actix-web` 4 stable version was never officially released. It was a pressing plan to revamp the technology stack, and I knew I had a chance when `poem` was released.

* I felt more flexibility than ever before in using `poem` and transparently relying on tokio.
I was able to replace some of the original actix-web components directly with components from the tokio ecosystem, and upgrade a lot of dependencies.
No more manual patching, or using old dependencies.

#### A brief background on `poem`. 1.

1. the framework has a very fast performance , consistent philosophy , and a clear implementation . 2.
2. based on `hyper`, combined with `tokio`, users have more control.


#### The migration focuses on.

1. regrouping of web components, different style of maintaining application state.

2. api-level modifications to avoid business logic adjustments.


Basic pre-migration grooming.

* The `handler` in poem is an `Endpoint` object that generates a `Future`, and the collaboration between the framework and `tokio` allows the request to be computed efficiently in a multi-threaded runtime.

   This is not the case with actix-web, which is internally composed of multiple single-threaded `Runtime`s.
   Because of this subtle difference, the `handler` previously used for actix-web cannot be used directly for `poem`, because it is necessary to ensure that each `handler` is used for the same request.
   Because of the need to ensure the input state of each `handler` and to ensure that the values across .await need to all Send.

* poem's routing is a nestable `Endpoint` data structure, unlike the original `actix-web` configuration.

* Most of poem's exposed data structures support Send, allowing efficient use of thread resources, as opposed to `actix-web`.

* All middleware implementations need to be modified, all backend Tasks need to be revamped, and all global state needs to be adjusted.

* Upgrade multiple dependencies with direct dependencies on `tokio` 1.0.

* Testing of the full link and writing migration chronicles.


### Here are some comparisons of `poem` & `actix-web`:

#### routing side
In the previous implementation based on `actix-web`, a large number of routing groups were registered via `configure`, application state was registered via `app_data`, and middleware was registered via `wrap`.
```
let app = App::new()
    .configure(actions::task::config)
    .configure(actions::user::config)
    .configure(actions::task_log::config)
    .configure(actions::executor_group::config)
    .configure(actions::executor_processor::config)
    .configure(actions::executor_processor_bind::config)
    .configure(actions::data_reports::config)
    .configure(actions::components::config)
    .configure(actions::operation_log::config)
    .configure(actions::user_login_log::config)
    .app_data(shared_delay_timer.clone())
    .app_data(shared_connection_pool.clone())
    .app_data(shared_scheduler_meta_info.clone())
    .wrap(components::session::auth_middleware())
    .wrap(components::session::session_middleware());
```

Example of routing configuration.
```
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(create_user)
        .service(show_users)
        .service(update_user)
        .service(delete_user)
        .service(login_user)
        .service(logout_user)
        .service(check_user)
        .service(change_password)
        .service(roles)
        .service(permissions)
        .service(append_permission)
        .service(delete_permission)
        .service(append_role)
        .service(delete_role);
}

```

Example of a `handler` processing request.
```
#[post("/api/user/create")]
async fn create_user(
    web::Json(user): web::Json<model::QueryNewUser>,
    pool: ShareData<db::ConnectionPool>,
) -> HttpResponse {

    // do someting.
}
```

Now based on the implementation of `poem`, a large number of route groups are organized by Route and can be multi-nested. Application state & middleware are registered via `with`, and all components have the common feature `Endpoint`.
```
let app = Route::new().nest_no_strip(
            "/api",
            Route::new()
                .nest_no_strip("/api/task", actions::task::route_config())
                .nest_no_strip("/api/user", actions::user::route_config())
                .nest_no_strip("/api/role", actions::role::route_config())
                .nest_no_strip("/api/task_log", actions::task_log::route_config())
                .nest_no_strip("/api/tasks_state", actions::data_reports::route_config())
                .nest_no_strip("/api/task_instance", actions::task_instance::route_config())
                .nest_no_strip("/api/binding", actions::components::binding::route_config())
                .nest_no_strip("/api/operation_log", actions::operation_log::route_config())
          )
          .with(shared_delay_timer)
          .with(shared_connection_pool)
          .with(shared_scheduler_meta_info)
          .with(shared_request_client)
          .with(components::session::auth_middleware())
          .with(components::session::cookie_middleware());

```

Example of routing configuration in `poem`.

```
pub fn route_config() -> Route {
    Route::new()
        .at("/api/user/create", post(create_user))
        .at("/api/user/list", post(show_users))
        .at("/api/user/update", post(update_user))
        .at("/api/user/delete", post(delete_user))
        .at("/api/user/login", post(login_user))
        .at("/api/user/logout", post(logout_user))
        .at("/api/user/check", post(check_user))
        .at("/api/user/change_password", post(change_password))
        .at("/api/user/roles", post(roles))
        .at("/api/user/permissions", post(permissions))
        .at("/api/user/append_permission", post(append_permission))
        .at("/api/user/delete_permission", post(delete_permission))
        .at("/api/user/append_role", post(append_role))
        .at("/api/user/delete_role", post(delete_role))
}
```

Example of `handler` processing requests in `poem`.
```
async fn create_user(
    web::Json(user): web::Json<model::QueryNewUser>,
    pool: ShareData<db::ConnectionPool>,
) -> HttpResponse {

    // do someting.
}
```

### Substitution of `poem` concepts:

#### handler
The handler in `poem`, which doesn't differ much from `actix-web`, only needs some `extractor` adjustments, and for some blocking tasks, switch to the `tokio` api to compute:

```
#[handler]

async fn show_task_log_detail(
    Json(query_params): Json<model::RecordId>,
    pool: Data<&Arc<db::ConnectionPool>>,
) -> impl IntoResponse {
    use db::schema::task_log_extend;

    if let Ok(conn) = pool.get() {
        let f_result = spawn_blocking::<_, Result<_, diesel::result::Error>>(move || {
            let task_log_extend = task_log_extend::table
                .find(query_params.record_id.0)
                .first::<model::TaskLogExtend>(&conn)?;

            Ok(task_log_extend)
        })
        .await;

        let log_extend = f_result
            .map(|log_extend_result| {
                Into::<UnifiedResponseMessages<model::TaskLogExtend>>::into(log_extend_result)
            })
            .unwrap_or_else(|e| {
                UnifiedResponseMessages::<model::TaskLogExtend>::error()
                    .customized_error_msg(e.to_string())
            });
        return Json(log_extend);
    }

    Json(UnifiedResponseMessages::<model::TaskLogExtend>::error())
}
```

#### Endpoint
`Endpoint` abstracts the HTTP request trait, and is the true face of all `handler`.

You can implement `Endpoint` to create your own `Endpoint` handler.

```
/// An HTTP request handler.
#[async_trait::async_trait]
pub trait Endpoint: Send + Sync {
    /// Represents the response of the endpoint.
    type Output: IntoResponse;

    /// Get the response to the request.
    async fn call(&self, req: Request) -> Self::Output;
}
```

`poem`'s `Endpoint` philosophy is very similar to `Service` in tower, but poem is more concise, and `poem` is also compatible with `tower` to reuse its ecology and components.
```
/// `Service` provides a mechanism by which the caller is able to coordinate
/// readiness. `Service::poll_ready` returns `Ready` if the service expects that
/// it is able to process a request.
pub trait Service<Request> {
    /// Responses given by the service.
    type Response;

    /// Errors produced by the service.
    type Error;

    /// The future response value.
    type Future: Future<Output = Result<Self::Response, Self::Error>>;

    /// Returns `Poll::Ready(Ok(()))` when the service is able to process 
    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>>;

    /// Process the request and return the response asynchronously.
    fn call(&mut self, req: Request) -> Self::Future;
}
```

#### IntoResponse
`IntoResponse` is an abstraction of the response data. 

All Response types that can be converted to HTTP responses should implement IntoResponse, and they can be used as return values for `handler`.

```
pub trait IntoResponse: Send {
    /// Consume itself and return [`Response`].
    fn into_response(self) -> Response;

    /// Wrap an `impl IntoResponse` to add a header.
    fn with_header<K, V>(self, key: K, value: V) -> WithHeader<Self>
    where
        K: TryInto<HeaderName>,
        V: TryInto<HeaderValue>,
        Self: Sized,
    {
        let key = key.try_into().ok();
        let value = value.try_into().ok();

        WithHeader {
            inner: self,
            header: key.zip(value),
        }
    }

    /// Wrap an `impl IntoResponse` to set a status code.
    fn with_status(self, status: StatusCode) -> WithStatus<Self>
    where
        Self: Sized,
    {
        WithStatus {
            inner: self,
            status,
        }
    }

    /// Wrap an `impl IntoResponse` to set a body.
    fn with_body(self, body: impl Into<Body>) -> WithBody<Self>
    where
        Self: Sized,
    {
        WithBody {
            inner: self,
            body: body.into(),
        }
    }
}

```

#### middleware

Making middleware with `poem` is very easy, here is an example of middlware that adds logger-id to a request.

```
// Unit-struct of logger-id for impl Middleware.
pub struct LoggerId;

impl<E: Endpoint> Middleware<E> for LoggerId {
    type Output = LoggerIdMiddleware<E>;

    fn transform(&self, ep: E) -> Self::Output {
        LoggerIdMiddleware { ep }
    }
}
// Wraps the original handler and logs the processing of the request internally.
pub struct LoggerIdMiddleware<E> {
    ep: E,
}

#[poem::async_trait]
impl<E: Endpoint> Endpoint for LoggerIdMiddleware<E> {
    type Output = E::Output;

    async fn call(&self, req: Request) -> Self::Output {
        let unique_id = get_unique_id_string();
        self.ep
            .call(req)
            .instrument(info_span!("logger-", id = unique_id.deref()))
            .await
    }
}

```

The following is a sample template for actix-web implementation of middlware, the template code is indeed slightly lengthy.

```
pub(crate) struct CasbinService;

impl<S, B> Transform<S> for CasbinService
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = ActixWebError>
        + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = ActixWebError;
    type InitError = ();
    type Transform = CasbinAuthMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(CasbinAuthMiddleware {
            service: Rc::new(RefCell::new(service)),
        })
    }
}

pub struct CasbinAuthMiddleware<S> {
    service: Rc<RefCell<S>>,
}


impl<S, B> Service for CasbinAuthMiddleware<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = ActixWebError>
        + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = ActixWebError;
    type Future = MiddlewareFuture<Self::Response, Self::Error>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        Box::pin(async move {

           // do something.
                return service.call(req).await;

        })
    }
```

## Summary

0. The migration involved 45 file changes and 4000 lines of code changes (2500 lines were added and 1579 lines were removed).

1. Switching to `poem` allowed the project to have unprecedented flexibility when upgrading old dependencies and transparently relying on the tokio ecosystem. No longer do you have to manually make your own patches or use obsolete dependencies.

2. With `poem` & `tokio` ecosystem in place after migration, it is easier to extend functionality and reduce maintenance costs.

3. better resource utilization and multi-core advantage without affecting performance metrics.


## Thanks

During the migration process, I had some requirements that could not be handled directly using `poem`.
Then I opened a few issues on `poem` and within a day I was able to communicate with the author and support the feature in `poem`, so powerful!

* I would like to thank the whole community and the code contributors.
Especially the author of `poem`:
[sunli829](https://github.com/sunli829)

Thank you!


* Thanks to users for reporting spelling errors in the documentation, which is greatly appreciated by everyone.

* Thanks to users for joining us, providing feedback, discussing features, and getting help!

* I also appreciate such a great work from the `actix-web` community, as I decided to migrate to `poem` due to technical choice issues.



### Repos:

[poem](https://github.com/poem-web/poem)

[delicate](https://github.com/BinChengZhao/delicate)

### Message:

##### I recently left my old job and am looking for a new one. If you are interested in my work and need a Rust Engineer, please contact me `binchengZhao@outlook.com` .
