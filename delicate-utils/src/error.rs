use crate::prelude::*;
use actix_web::client::JsonPayloadError;
use actix_web::client::SendRequestError as ClientSendRequestError;
use actix_web::error::{BlockingError, Error as ActixWebError};
use diesel::r2d2::PoolError;

#[derive(ThisError, Debug)]
pub enum CommonError {
    #[error("db connect fail.")]
    DisConnDb(#[from] PoolError),
    #[error("redis connect fail.")]
    DisConnRedis(#[from] redis::RedisError),
    #[error("data query fail.")]
    DisQuery(#[from] diesel::result::Error),
    #[error("data blocking-query fail.")]
    DisBlockingQuery(#[from] BlockingError<diesel::result::Error>),
    #[error("data send fail.")]
    DisSend(#[from] ClientSendRequestError),
    #[error("data parse-json fail.")]
    DisParse(#[from] JsonPayloadError),
    #[error("data serializing fail.")]
    DisSer(#[from] serde_json_error::Error),
    #[error("data sign or decrypt or verify fail.")]
    DisSign(#[from] ras_error::Error),
    #[error("Consensus message signature verification failed.")]
    DisVerify,
    #[error("DelayTimer's task operation failed.")]
    DisOpeate(#[from] TaskError),
    #[error("Invalid operation, or invalid data.(`{0}`)")]
    DisPass(String),
    #[error("The errors reported by the framework cannot be ignored.")]
    DisAccept(#[from] ActixWebError),
    #[error("The errors reported by the auth-service cannot be ignored.")]
    DisAuth(#[from] AuthServiceError),
}

#[derive(ThisError, Debug)]
pub enum AuthServiceError {
    #[error("db connect fail.")]
    DisConnDb(#[from] PoolError),
    #[error("data query fail.")]
    DisQuery(#[from] diesel::result::Error),
    #[error("data blocking-query fail.")]
    DisBlockingQuery(#[from] BlockingError<diesel::result::Error>),
    #[error("The errors reported by the casbin is Authentication-related errors.")]
    DisAuthCasbin(#[from] casbin::error::Error),
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
