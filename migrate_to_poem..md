`delicate` Project Background.

 Distributed scheduling system. [Repo](https://github.com/BinChengZhao/delicate)

1. 5w lines of code + documentation, etc.
2. main language is Rust + js.

The migration involved 50 file changes and 3500 lines of code changes, which were completed in 2 days.

For more details please check: xxx

Why migrate to `poem`?

The iterative progress of actix-web is not keeping up with the current needs.

Brief background on `poem`.

1. the highly skilled and passionate author, `sunli`.
2. based on `hyper`, combined with `tokio` users have more control.


The focus of the migration.

1. recombination of web components. 

2. api level modifications to avoid business logic adjustments.

etc.....


*. handle in poem, is an asynchronous state machine, using tokio it can be computed efficiently in a multi-threaded runtime.
   This is not the case with actix-web, which is single-threaded. Because of this subtle difference, the handle previously used for actix-web
   The difference here is that the handle previously used for actix-web cannot be used for `poem`, because the values across .await need to be kept in Send, which is a lot of work.

*. poem's route, a root route, and then multiple mods constantly at inside, different from the original actix-web config.

*. poem's error supports Send, actix-web's error does not support Send, which makes cross-threading difficult.

Need to modify all middleware implementations, need to revamp all handlers, need to adjust all global state.

I am also grateful to the actix-web community for such a great piece of work, and I decided to migrate to `poem` because of technical selection issues.

Translated with www.DeepL.com/Translator (free version)


// actix-web
pub(crate) fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(binding_list)
        .service(executor_list)
        .service(permission_list);
}

// poem
pub(crate) fn config_route(route: Route) -> Route {
    route
        .at("/api/binding/list", get(binding_list))
        .at("/api/executor/list", get(executor_list))
        .at("/api/permission/list", get(permission_list))
}

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
            .configure(actions::user_login_log::config);

        let app = Some(Route::new())
            .map(actions::task::config_route)
            .map(actions::user::config_route)
            .map(actions::task_log::config_route)
            .map(actions::executor_group::config_route)
            .map(actions::executor_processor::config_route)
            .map(actions::executor_processor_bind::config_route)
            .map(actions::data_reports::config_route)
            .map(actions::components::config_route)
            .map(actions::operation_log::config_route)
            .map(actions::user_login_log::config_route)
            .expect("");

// remove patch and older packages.

// actix-web RequestClient -> reqwest Client  

# TODO: This(casbin-patch) must be remove when upgrading to actix-web4.
[patch.crates-io]
casbin = { git = 'https://github.com/BinChengZhao/casbin-rs', branch = 'fix-casbin-v2.0.5-compile-bug' }

            # TODO: This(casbin-version) must be changed when upgrading to poem.
casbin = {version = '2.0.5', default-features = false, features = ["incremental", "logging", "tokio", "runtime-tokio", "watcher"]}


# TODO: This(redis-version) must be changed when upgrading to poem.
redis = { version = "= 0.17.0", features = ["connection-manager", "tokio-comp"] }

 Cookie 中间件，是否能支持用户自定义配置属性？
 目前使用CookieJar，每存一个新Cookie都需要手动配置安全相关的属性，这虽然很灵活，但是有很多重复的工作在做。

 对一个站点，通常cookie的安全策略是保持一致的，对每个key都单独配置的场景比较少。

 并且目前`poem`中，Cookie中间件是开启feature后自动注册的，这样用户缺少一些初始化属性的控制力。

 我建议：Cookie 中间件，用户在框架启动时支持自定义配置并且可以覆盖默认装载的Cookie中间件，
 后续 Cookie 默认走配置的安全属性，使用起来会很轻便优雅。


如何在中间件中提前响应？

我最近在迁移项目从 `actix-web` 到 `poem` 遇到了一些棘手的问题.

问题描述：

在 `actix-web` 中，我可以将任务分成两步：

1.状态判断成功 -> service.call() (相似于`poem`中的 ep.call())
2. 状态判断失败，提前响应 -> 通过 req.error_response() (ServiceRequest::error_response)

步骤1可以在poem中实现，但是步骤2目前没找到好的办法。

参考example中的中间件实现，可以用 extensions 加一个 `状态X`
handler 中使用提取器提取 `Result<状态X>` 自己处理,
但是我有50/60个 handler，不方便给每个handler都加一个状态。
并且每次加中间件，都需要给handler配套加 `状态*`, 会让handler 很冗长。 

特定向您请教。

Box::pin(async move {
                        Ok(req.error_response(
                            HttpResponseBuilder::new(StatusCode::default()).json(
                                UnifiedResponseMessages::<()>::error().customized_error_msg(
                                    String::from("Please log in and operate."),
                                ),
                            ),
                        ))
                    })



##### I've been away from work for a while, and I'm ready to find a new job. If you are interested in my work or need a Rust engineer, please contact me `binchengZhao@outlook.com` .