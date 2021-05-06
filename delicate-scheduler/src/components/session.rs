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
