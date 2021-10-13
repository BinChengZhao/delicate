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