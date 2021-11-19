use super::prelude::*;
// Register the actual session middleware that is used to maintain session
// state.

// `CookieSession` is an actual session processing backend
// that does the initialization of the state of the `Session` instance inside
// the application (ServiceRequest::get_session) when the request is received.

// And when the request is processed `CookieSession` gets the latest state
// content in the `Session` and sets it to the client
// (all this is done in the middleware of `CookieSession`).
pub(crate) fn session_middleware() -> CookieSession {
    // Be sure to use a uniform `SESSION_TOKEN` here,
    // If each server generates a random key, it will cause inconsistency and the
    // login status will continue to fail.
    let token_bytes =
        env::var("SESSION_TOKEN").expect("Without `SESSION_TOKEN` set in .env").into_bytes();

    let cookie_config = CookieConfig::signed(CookieKey::derive_from(&token_bytes))
        .domain(
            env::var("SCHEDULER_COOKIE_DOMAIN")
                .expect("Without `SCHEDULER_COOKIE_DOMAIN` set in .env"),
        )
        .name(env::var("SCHEDULER_NAME").expect("Without `SCHEDULER_NAME` set in .env"))
        .http_only(true)
        .secure(false);
    CookieSession::new(cookie_config)
}

pub(crate) fn cookie_middleware() -> CookieJarManager {
    // Be sure to use a uniform `SESSION_TOKEN` here,
    // If each server generates a random key, it will cause inconsistency and the
    // login status will continue to fail.
    let token_bytes =
        env::var("SESSION_TOKEN").expect("Without `SESSION_TOKEN` set in .env").into_bytes();

    CookieJarManager::with_key(CookieKey::derive_from(&token_bytes))
}

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

    async fn call(&self, req: Request) -> Self::Output {
        #[cfg(APP_DEBUG_MODE)]
        {
            return self.ep.call(req).await.into_response();
        }

        let session = req.session();
        let uri = req.uri();
        let path = uri.path();

        // Use `CookieJar` as the backend of `Session`.
        // Judgment, if it is a special api will not check the token.
        // (for example: login-api, event-collection-api)

        match path {
            "/api/user/login" | "/api/task_log/event_trigger" => {
                self.ep.call(req).await.into_response()
            },
            _ => {
                let user_id = session.get::<u64>("user_id");
                if user_id.is_some() {
                    self.ep.call(req).await.into_response()
                } else {
                    UnifiedResponseMessages::<()>::error()
                        .customized_error_msg(String::from("Please log in and operate."))
                        .into_response()
                }
            },
        }
    }
}
