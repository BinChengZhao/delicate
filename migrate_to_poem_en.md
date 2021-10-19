## Title: Migration of Rust production projects from `actix-web` to `poem`.

### Some background on the `delicate` project.

[delicate](https://github.com/BinChengZhao/delicate) A lightweight, distributed task scheduling platform. 


<a href="">
    <img src="https://delicate-rs-1301941387.cos.ap-beijing.myqcloud.com/delicate-rs/delicate_logo.png"
         alt="delicate logo" title="delicate" height="125" width="125" align="right"/>
</a>


1. project size: 5w lines of code + documentation.
2. main language is Rust + js. 3.
3. The migration involved 45 file changes and 4000 lines of code changes (2500 lines added and 1579 lines removed).

### Technology stack

* Backend ( scheduler & executor ): Rust  

* Original main dependencies: (actix-web & diesel & delay-timer & serde & tracing)

* Current main dependencies: (poem & tokio & diesel & delay-timer & serde & tracing)

* Frontend: antd-admin (React)

* Ui: Ant Design

* Database: mysql , postgres (plan support)


### Why migrate to `poem`?

* While using actix-web, because actix-web 4 stable version, was never officially released and I wanted to use a library compatible with `tokio` 1.0 has been an urgent problem to solve, when `poem` was released I knew the opportunity was here.

* I felt more flexibility than ever before in using `poem` and transparently relying on tokio.
I was able to replace some of the original actix-web components directly with tokio eco-components, and upgrade a lot of dependencies.
No more manual patching, or using old dependencies.

#### A brief background on `poem`.

1. the framework has a very fast performance , consistent philosophy , and a clear implementation .
2. based on `hyper`, combined with `tokio`, users have more control.


#### The migration focuses on.

1. regrouping of web components, different style of maintaining application state.

2. api-level modifications to avoid business logic adjustments.


#### Basic pre-migration grooming.

* `handler` in poem generates a `Future` and the collaboration between the framework and `tokio` allows the request to be computed efficiently in a multi-threaded runtime.

   This is not the case with actix-web, which is internally composed of multiple single-threaded `Runtime`s.
   Because of this subtle difference, the `handler` previously used for actix-web cannot be used directly for `poem`, because it is necessary to ensure that each `handler` is used for the same request.
   Because of the need to ensure the input state of each `handler` and to ensure that the values across .await need to all Send.

* poem's routing is a nestable `Endpoint` data structure, unlike the original actix-web configuration.

* Most of poem's exposed data structures support Send, allowing efficient use of thread resources, as opposed to `actix-web`.

* All middleware implementations need to be modified, all backend Tasks need to be revamped, and all global state needs to be adjusted.

* Upgrade multiple dependencies with direct dependencies on `tokio` 1.0.

* Testing of the full link and writing migration chronicles.


### Here are some `poem` & `actix-web` comparisons:

#### routing side
Previous implementation based on `actix-web`, with a large number of routing groups going through configure to register.
![actix-app](. /doc/src/_media/migrate_to_poem/actix_app.png)
![actix-config](. /doc/src/_media/migrate_to_poem/actix_config.png)

Now based on the `poem` implementation, a large number of route groups are organized by routes, which can be multi-nested: !
![poem-routes](. /doc/src/_media/migrate_to_poem/poem_routes.png)
![poem-route-config](. /doc/src/_media/migrate_to_poem/poem_route_config.png)

It is also possible to make a root Route to constantly at.
![poem-app](. /doc/src/_media/migrate_to_poem/poem_app.png)
![poem-config](. /doc/src/_media/migrate_to_poem/poem_config.png)

#### handler
The handler in `poem`, not much different from the original one, just need to adjust some `extractor`, for some blocking task, switch to tokio's api to compute
![poem-handler](. /doc/src/_media/migrate_to_poem/poem_handler.png)


#### Endpoint
`Endpoint` abstracts the HTTP request trait.

You can implement `Endpoint` to create your own `Endpoint` handler.
Here is the definition of `Endpoint`:
![poem-endpoint](. /doc/src/_media/migrate_to_poem/poem_endpoint.png)

The `Endpoint` philosophy of `poem` is very similar to that of `Service` in tower, but poem is a bit more concise, and `poem` is also compatible with `tower` to reuse its ecology and components.
![tower-service](. /doc/src/_media/migrate_to_poem/tower_service.png)

#### IntoResponse
`IntoResponse` is an abstraction of the response data. 

All Response types that can be converted to HTTP responses should implement IntoResponse, and they can be used as return values for `handler`.
![poem-into-response](. /doc/src/_media/migrate_to_poem/poem_into_response.png)

#### middleware

Making middleware with `poem` is very easy, here is an example of middlware that adds logger-id to a request
![poem-middleware-logger](. /doc/src/_media/migrate_to_poem/poem_middleware_logger.png)

The following is a sample template for the actix-web implementation of middlware, the template code is indeed a bit long and intriguing.
![actix-middlware](. /doc/src/_media/migrate_to_poem/actix_middlware.png)


## Thanks

During the migration process, I had some requirements that could not be handled directly using `poem`
Then I opened a few issues on `poem` and within a day I was able to communicate with the author and support the feature in `poem`.

I would like to thank the whole community and the code contributors. Especially the author of `poem`:
[sunli829](https://github.com/sunli829)

Thank you!


Thanks to users for reporting spelling errors in the documentation, which is greatly appreciated by everyone.
Thanks to users for joining us, providing feedback, discussing features, and getting help!
I also appreciate such a good work from the `actix-web` community, as I decided to migrate to `poem` due to technical selection issues.



### Repos:

[poem](https://github.com/poem-web/poem)

[delicate](https://github.com/BinChengZhao/delicate)

### Message:

##### I recently left my old job and am looking for a new one. If you are interested in my job and need a Rust engineer, please contact me `binchengZhao@outlook.com` .