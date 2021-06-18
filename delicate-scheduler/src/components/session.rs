use super::prelude::*;

// Register the actual session middleware that is used to maintain session state.

// `CookieSession` is an actual session processing backend
// that does the initialization of the state of the `Session` instance inside the application (ServiceRequest::get_session)
// when the request is received.

// And when the request is processed `CookieSession` gets the latest state content
// in the `Session` and sets it to the client
// (all this is done in the middleware of `CookieSession`).
pub(crate) fn session_middleware() -> CookieSession {
    CookieSession::signed(
        &env::var("SESSION_TOKEN")
            .expect("Without `SESSION_TOKEN` set in .env")
            .into_bytes(),
    )
    .domain(env::var("SCHEDULER_DOMAIN").expect("Without `SCHEDULER_DOMAIN` set in .env"))
    .name(env::var("SCHEDULER_NAME").expect("Without `SCHEDULER_NAME` set in .env"))
}

// Register authentication middleware to check login status based on `CookieSession`.
pub(crate) fn auth_middleware() -> SessionAuth {
    SessionAuth
}

// The public middleware output type.
type MiddlewareFuture<T, E> = Pin<Box<dyn Future<Output = Result<T, E>>>>;

pub struct SessionAuth;

impl<S, B> Transform<S> for SessionAuth
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = ActixWebError>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = ActixWebError;
    type InitError = ();
    type Transform = SessionAuthMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(SessionAuthMiddleware { service })
    }
}

pub struct SessionAuthMiddleware<S> {
    service: S,
}

impl<S, B> Service for SessionAuthMiddleware<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = ActixWebError>,
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
        let session = req.get_session();
        let uri = req.uri();
        let path = uri.path();

        // Judgment, if it is a special api will not check the token.
        // (for example: login-api, event-collection-api)

        match path {
            "/api/user/login" | "/api/task_logs/event_trigger" => {
                let fut = self.service.call(req);
                Box::pin(async move {
                    let res = fut.await?;
                    Ok(res)
                })
            }
            _ => {
                if let Ok(Some(_)) = session.get::<String>("user_id") {
                    let fut = self.service.call(req);
                    Box::pin(async move {
                        let res = fut.await?;
                        Ok(res)
                    })
                } else {
                    Box::pin(async move {
                        Ok(req.error_response(
                            HttpResponseBuilder::new(StatusCode::default()).json(
                                UnifiedResponseMessages::<()>::error().customized_error_msg(
                                    String::from("Please log in and operate."),
                                ),
                            ),
                        ))
                    })
                }
            }
        }
    }
}
