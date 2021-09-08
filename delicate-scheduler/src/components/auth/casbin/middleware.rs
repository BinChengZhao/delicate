#![allow(unused_imports)]
use crate::prelude::*;

lazy_static! {

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

      // Because casbin requires that the `&str` type must satisfy static.
      // And the String read by environment variable does not satisfy this condition after passing deref.
      // Two ways to solve it.
      // 1. Active memory leak.
      // 2. Assign the value read by environment variable to static variable,
      // Then the reference of static variable can satisfy the static restriction.
    pub static ref AUTHER: RwLock<Enforcer> = {

        let e = futures_block_on(Enforcer::new((&CASBIN_MODEL_CONF_PATH).deref().deref(), (&CASBIN_POLICY_CONF_PATH).deref().deref()))
            .expect("Unable to read permission file.");
        RwLock::new(e)
    };

}

#[allow(dead_code)]
pub(crate) async fn warm_up_auther() {
    AUTHER.write().await.enable_log(true);
}

#[allow(dead_code)]
pub(crate) async fn get_auther_read_guard() -> RwLockReadGuard<'static, Enforcer> {
    AUTHER.read().await
}

#[allow(dead_code)]
pub(crate) async fn get_auther_write_guard() -> RwLockWriteGuard<'static, Enforcer> {
    AUTHER.write().await
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
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

#[derive(Clone)]

pub struct CasbinAuthMiddleware<S> {
    service: Rc<RefCell<S>>,
}

const WHITE_LIST: [&str; 8] = [
    "/api/tasks_state/one_day",
    "/api/user/login",
    "/api/user/logout",
    "/api/binding/list",
    "/api/user/check",
    "/api/executor/list",
    "/api/user/change_password",
    "/api/task_logs/event_trigger",
];

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
        // Because also impl<S> Service for Rc<RefCell<S>> in actix.
        // So it work.
        self.service.poll_ready(cx)
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        let mut service = self.service.clone();
        let session = req.get_session();
        let path = req.path().to_string();
        let auth_part = path.split('/').into_iter().collect::<Vec<&str>>();

        let resource = auth_part.get(2).map(|s| s.to_string()).unwrap_or_default();
        let action = auth_part.get(3).map(|s| s.to_string()).unwrap_or_default();
        let username = session
            .get::<String>("user_name")
            .unwrap_or_default()
            .unwrap_or_default();

        Box::pin(async move {
            // Path in the whitelist do not need to be verified.
            if WHITE_LIST.contains(&path.deref()) {
                return service.call(req).await;
            }

            let auther = get_auther_read_guard().await;

            if username.is_empty() || resource.is_empty() || action.is_empty() {
                return Ok(req.error_response(
                    HttpResponseBuilder::new(StatusCode::default()).json(
                        UnifiedResponseMessages::<()>::error()
                            .customized_error_msg(String::from("Permission check failed.")),
                    ),
                ));
            }

            match auther.enforce(vec![username, resource, action]) {
                Ok(true) => {
                    drop(auther);
                    service.call(req).await
                }
                Ok(false) => {
                    drop(auther);
                    Ok(req.error_response(
                        HttpResponseBuilder::new(StatusCode::default()).json(
                            UnifiedResponseMessages::<()>::error()
                                .customized_error_msg(String::from("Permission check failed.")),
                        ),
                    ))
                }
                Err(e) => {
                    drop(auther);
                    Ok(req.error_response(
                        HttpResponseBuilder::new(StatusCode::default()).json(
                            UnifiedResponseMessages::<()>::error()
                                .customized_error_msg(format!("Permission check failed. ({})", e)),
                        ),
                    ))
                }
            }
        })
    }
}
