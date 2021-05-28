use super::prelude::*;

pub(crate) fn session_middleware() -> CookieSession {
    CookieSession::signed(
        &env::var("SESSION_TOKEN")
            .expect("Without `SESSION_TOKEN` set in .env")
            .into_bytes(),
    )
    .domain(env::var("SCHEDULER_DOMAIN").expect("Without `SCHEDULER_DOMAIN` set in .env"))
    .name(env::var("SCHEDULER_NAME").expect("Without `SCHEDULER_NAME` set in .env"))
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

        if let Ok(Some(_token)) = session.get::<String>("token") {
            println!("Hi from start. You requested: {}", req.path());

            let fut = self.service.call(req);

            Box::pin(async move {
                let res = fut.await?;

                println!("Hi from response");
                Ok(res)
            })
        } else {
            Box::pin(async move {
                Ok(req.error_response(HttpResponseBuilder::new(StatusCode::default())))
            })
        }
    }
}
