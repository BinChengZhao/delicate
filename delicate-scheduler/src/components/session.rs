use super::prelude::*;
// Register the actual session middleware that is used to maintain session state.

// `CookieSession` is an actual session processing backend
// that does the initialization of the state of the `Session` instance inside the application (ServiceRequest::get_session)
// when the request is received.

// And when the request is processed `CookieSession` gets the latest state content
// in the `Session` and sets it to the client
// (all this is done in the middleware of `CookieSession`).
// pub(crate) fn session_middleware() -> CookieSession {
//     CookieSession::signed(
//         &env::var("SESSION_TOKEN")
//             .expect("Without `SESSION_TOKEN` set in .env")
//             .into_bytes(),
//     )
//     .domain(
//         env::var("SCHEDULER_COOKIE_DOMAIN").expect("Without `SCHEDULER_COOKIE_DOMAIN` set in .env"),
//     )
//     .name(env::var("SCHEDULER_NAME").expect("Without `SCHEDULER_NAME` set in .env"))
//     .http_only(true)
//     .secure(false)
// }

// Register authentication middleware to check login status based on `CookieSession`.

pub(crate) fn auth_middleware() -> SessionAuth {
    SessionAuth
}

pub struct SessionAuth;

impl<E: Endpoint> Middleware<E> for SessionAuth {
    type Output = SessionAuthMiddleware<E>;

    fn transform(&self, ep: E) -> Self::Output {
        SessionAuthMiddleware { ep }
    }
}

pub struct SessionAuthMiddleware<E> {
    ep: E,
}

#[poem::async_trait]
impl<E: Endpoint> Endpoint for SessionAuthMiddleware<E> {
    type Output = Response;

    async fn call(&self, mut req: Request) -> Self::Output {
        #[cfg(APP_DEBUG_MODE)]
        {
            return self.ep.call(req).await.into_response();
        }

        let extensions = req.extensions();
        let session = extensions.get::<CookieJar>();
        let uri = req.uri();
        let path = uri.path();

        // Use `CookieJar` as the backend of `Session`.
        // Judgment, if it is a special api will not check the token.
        // (for example: login-api, event-collection-api)

        match path {
            "/api/user/login" | "/api/task_log/event_trigger" => {
                self.ep.call(req).await.into_response()
            }
            _ => {
                let user_id = session
                    .map(|s| s.get("user_id"))
                    .flatten()
                    .map(|c| c.value::<u64>());
                if let Some(Ok(_)) = user_id {
                    self.ep.call(req).await.into_response()
                } else {
                    UnifiedResponseMessages::<()>::error()
                        .customized_error_msg(String::from("Please log in and operate."))
                        .into_response()
                }
            }
        }
    }
}
