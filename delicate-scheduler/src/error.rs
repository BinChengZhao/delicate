use crate::prelude::*;
use actix_web::client::JsonPayloadError;
use actix_web::error::BlockingError;
use diesel::r2d2::PoolError;
#[derive(ThisError, Debug)]
pub enum BindExecutorError {
    #[error("db connect fail.")]
    DisConn(#[from] PoolError),
    #[error("data query fail.")]
    DisQuery(#[from] BlockingError<diesel::result::Error>),
    #[error("data sign or Decrypt fail.")]
    DisSign(#[from] ras_error::Error),
    #[error("data parse fail.")]
    DisParse(#[from] JsonPayloadError),
    #[error("data send fail.")]
    DisSend(#[from] ClientSendRequestError),
    #[error("data serializing fail.")]
    DisSer(#[from] serde_json_error::Error),
}

#[derive(ThisError, Debug)]
pub enum InitSchedulerError {
    #[error("Environment Variables `{0}` missed.")]
    MisEnvVar(String),
    #[error("Access fileSystem fail.")]
    DisAccessFs(#[from] std::io::Error),
    #[error("Parse pem file fail.")]
    DisParsePem(#[from] pem::PemError),
    #[error("Parse pem to Key fail.")]
    DisParseKey(#[from] ras_error::Error),
}
