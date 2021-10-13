#![allow(unused_imports)]
use crate::prelude::*;
use adapter::adapter_core::DieselAdapter;

lazy_static! {

      // Because casbin(`Enforcer::new`) requires that the `&str` type must satisfy static.
      // And the String read by environment variable does not satisfy this condition after passing deref.
      // Two ways to solve it.
      // 1. Active memory leak.
      // 2. Assign the value read by environment variable to static variable,
      // Then the reference of static variable can satisfy the static restriction.

    pub static ref CASBIN_MODEL_CONF_PATH: String = {
            env::var("CASBIN_MODEL_CONF").expect("CASBIN_MODEL_CONF must be set")
    };


    // TODO: Can be listened to by `hotwatch` for changes.
    // TODO: `redis` based publish-subscribe, do real-time permission information synchronization.

    // TODO: Adjustments for permissions, like the operation log consumer, have a unique channel that
    // TODO: holds information about the operation and controls the flow of consumption.

    // TODO: Permissions editing is optional and
    // TODO: By default only supports initialization from the file once and does not support editing if it is not selected.

    pub static ref CASBIN_POLICY_CONF_PATH: String = {
            env::var("CASBIN_POLICY_CONF").expect("CASBIN_POLICY_CONF must be set")
    };


}

pub(crate) struct CasbinGuard;

impl CasbinWatcher for CasbinGuard {
    fn set_update_callback(&mut self, _cb: Box<dyn FnMut() + Send + Sync>) {
        error!(target:"set_update_callback", "unreachable.");
    }

    fn update(&mut self, d: EventData) {
        debug!("CasbinGuard: {}", &d);
        handle_event_for_watcher(d);
    }
}

#[allow(dead_code)]
pub(crate) async fn get_casbin_enforcer(pool: Arc<db::ConnectionPool>) -> Enforcer {
    let adapter = DieselAdapter::new(pool);
    let mut enforcer = Enforcer::new(get_casbin_model_conf_path(), adapter)
        .await
        .expect("Casbin's enforcer initialization error.");

    enforcer.set_watcher(Box::new(CasbinGuard));

    enforcer
}

#[allow(dead_code)]
pub(crate) fn get_casbin_model_conf_path() -> &'static str {
    CASBIN_MODEL_CONF_PATH.deref().deref()
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub(crate) struct CasbinService;

impl<E: Endpoint> Middleware<E> for CasbinService {
    type Output = CasbinAuthMiddleware<E>;

    fn transform(&self, ep: E) -> Self::Output {
        CasbinAuthMiddleware { ep }
    }
}

#[derive(Clone)]

pub struct CasbinAuthMiddleware<E> {
    ep: E,
}

const WHITE_LIST: [&str; 9] = [
    "/api/tasks_state/one_day",
    "/api/user/login",
    "/api/user/logout",
    "/api/binding/list",
    "/api/user/check",
    "/api/executor/list",
    "/api/user/change_password",
    "/api/task_log/event_trigger",
    "/api/casbin/test",
];

#[poem::async_trait]
impl<E: Endpoint> Endpoint for CasbinAuthMiddleware<E> {
    type Output = E::Output;

    async fn call(&self, mut req: Request) -> Self::Output {
        let enforcer = req
            .extensions()
            .get::<Data<Arc<RwLock<Enforcer>>>>()
            .expect("Casbin's enforcer acquisition failed")
            .clone();
        let extensions = req.extensions();
        let session = extensions.get::<CookieJar>();
        let path = req.uri().path().to_string();
        let auth_part = path.split('/').into_iter().collect::<Vec<&str>>();

        let resource = auth_part.get(2).map(|s| s.to_string()).unwrap_or_default();
        let action = auth_part.get(3).map(|s| s.to_string()).unwrap_or_default();

        // FIXME:
        todo!();
        // let username = session
        //     .get::<String>("user_name")
        //     .unwrap_or_default()
        //     .unwrap_or_default();
        // Box::pin(async move {
        //     // Path in the whitelist do not need to be verified.
        //     if WHITE_LIST.contains(&path.deref()) {
        //         return service.call(req).await;
        //     }

        //     #[cfg(APP_DEBUG_MODE)]
        //     {
        //         return service.call(req).await;
        //     }

        //     let auther = enforcer.read().await;

        //     if username.is_empty() || resource.is_empty() || action.is_empty() {
        //         return Ok(req.error_response(
        //             HttpResponseBuilder::new(StatusCode::default()).json(
        //                 UnifiedResponseMessages::<()>::error()
        //                     .customized_error_msg(String::from("Permission check failed.")),
        //             ),
        //         ));
        //     }

        //     match auther.enforce(vec![username, resource, action]) {
        //         Ok(true) => {
        //             drop(auther);
        //             service.call(req).await
        //         }
        //         Ok(false) => {
        //             drop(auther);
        //             Ok(req.error_response(
        //                 HttpResponseBuilder::new(StatusCode::default()).json(
        //                     UnifiedResponseMessages::<()>::error()
        //                         .customized_error_msg(String::from("Permission check failed.")),
        //                 ),
        //             ))
        //         }
        //         Err(e) => {
        //             drop(auther);
        //             Ok(req.error_response(
        //                 HttpResponseBuilder::new(StatusCode::default()).json(
        //                     UnifiedResponseMessages::<()>::error()
        //                         .customized_error_msg(format!("Permission check failed. ({})", e)),
        //                 ),
        //             ))
        //         }
        //     }
        // })
    }
}
