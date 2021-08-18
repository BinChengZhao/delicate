#![allow(unused_imports)]
use crate::prelude::*;

lazy_static! {
    pub static ref AUTHER: RwLock<Enforcer> = {
        let e = futures_block_on(Enforcer::new(
            "examples/rbac_with_domains_model.conf",
            "examples/rbac_with_domains_policy.csv",
        ))
        .expect("Unable to read permission file.");
        RwLock::new(e)
    };
}

pub(crate) async fn warm_up_auther() {
    AUTHER.write().await.enable_log(true);
}

pub(crate) async fn get_auther_read_guard() -> RwLockReadGuard<'static, Enforcer> {
    AUTHER.read().await
}

#[allow(dead_code)]
pub(crate) async fn get_auther_write_guard() -> RwLockWriteGuard<'static, Enforcer> {
    AUTHER.write().await
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct CasbinService;

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
        let mut service = self.service.clone();
        let session = req.get_session();
        let path = req.path().to_string();
        let action = req.method().as_str().to_string();
        let username = session
            .get::<String>("user_name")
            .unwrap_or_default()
            .unwrap_or_default();
        Box::pin(async move {
            let auther = get_auther_read_guard().await;

            if !username.is_empty() {
                match auther.enforce(vec![username, path, action]) {
                    Ok(true) => {
                        drop(auther);
                        service.call(req).await
                    }
                    Ok(false) => {
                        drop(auther);
                        todo!();
                    }
                    Err(_) => {
                        drop(auther);
                        todo!();
                    }
                }
            } else {
                todo!();
            }
        })
    }
}
