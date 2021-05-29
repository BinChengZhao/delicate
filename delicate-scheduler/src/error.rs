use crate::prelude::*;
use actix_web::client::JsonPayloadError;
use actix_web::error::BlockingError;
use diesel::r2d2::PoolError as PoolError;
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
