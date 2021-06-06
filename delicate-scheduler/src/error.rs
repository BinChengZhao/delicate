use crate::prelude::*;
use actix_web::client::JsonPayloadError;
use actix_web::error::BlockingError;
use diesel::r2d2::PoolError;

#[derive(ThisError, Debug)]
pub enum CommonError {
    #[error("db connect fail.")]
    DisConn(#[from] PoolError),
    #[error("data query fail.")]
    DisQuery(#[from] BlockingError<diesel::result::Error>),
    #[error("data send fail.")]
    DisSend(#[from] ClientSendRequestError),
    #[error("data parse-json fail.")]
    DisParse(#[from] JsonPayloadError),
    #[error("data serializing fail.")]
    DisSer(#[from] serde_json_error::Error),
    #[error("data sign or Decrypt fail.")]
    DisSign(#[from] ras_error::Error),
}
