use super::prelude::*;

pub(crate) fn logger_id_middleware() -> LoggerId {
    LoggerId
}

#[derive(Copy, Clone, Debug)]
pub struct LoggerId;

impl<E: Endpoint> Middleware<E> for LoggerId {
    type Output = LoggerIdMiddleware<E>;

    fn transform(&self, ep: E) -> Self::Output {
        LoggerIdMiddleware { ep }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct LoggerIdMiddleware<E> {
    ep: E,
}

#[poem::async_trait]
impl<E: Endpoint> Endpoint for LoggerIdMiddleware<E> {
    type Output = E::Output;

    async fn call(&self, req: Request) -> PoemResult<Self::Output> {
        let unique_id = get_unique_id_string();
        self.ep.call(req).instrument(info_span!("logger-", id = unique_id.deref())).await
    }
}
