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
    .domain(
        env::var("SCHEDULER_COOKIE_DOMAIN").expect("Without `SCHEDULER_COOKIE_DOMAIN` set in .env"),
    )
    .name(env::var("SCHEDULER_NAME").expect("Without `SCHEDULER_NAME` set in .env"))
    .http_only(true)
    .secure(false)
}

// Register authentication middleware to check login status based on `CookieSession`.
pub(crate) fn auth_middleware() -> SessionAuth {
    SessionAuth
}

pub struct SessionAuth;

impl<E: Endpoint> Middleware<E> for SessionAuth {
    type Output = SessionAuthMiddleware<E>;

    fn transform(&self, service: E) -> Self::Output {
        SessionAuthMiddleware { service }
    }
}

pub struct SessionAuthMiddleware<E> {
    service: E,
}

#[poem::async_trait]
impl<E: Endpoint> Endpoint for SessionAuthMiddleware<E> {
    type Output = E::Output;

    async fn call(&self, mut req: Request) -> Self::Output {
        // #[cfg(APP_DEBUG_MODE)]
        // {
        //     let fut = self.service.call(req);
        //     return Box::pin(async move {
        //         let res = fut.await?;
        //         Ok(res)
        //     });
        // }
        // let session = req.get_session();
        // let uri = req.uri();
        // let path = uri.path();

        // // Judgment, if it is a special api will not check the token.
        // // (for example: login-api, event-collection-api)

        // match path {
        //     "/api/user/login" | "/api/task_log/event_trigger" => {
        //         let fut = self.service.call(req);
        //         Box::pin(async move {
        //             let res = fut.await?;
        //             Ok(res)
        //         })
        //     }
        //     _ => {
        //         if let Ok(Some(_)) = session.get::<u64>("user_id") {
        //             let fut = self.service.call(req);
        //             Box::pin(async move {
        //                 let res = fut.await?;
        //                 Ok(res)
        //             })
        //         } else {
        //             Box::pin(async move {
        //                 Ok(req.error_response(
        //                     Json(
        //                         UnifiedResponseMessages::<()>::error().customized_error_msg(
        //                             String::from("Please log in and operate."),
        //                         ),
        //                     ),
        //                 ))
        //             })
        //         }
        //     }
        // }
        todo!();
    }
}
