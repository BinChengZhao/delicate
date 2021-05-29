use crate::prelude::*;
#[derive(ThisError, Debug)]
pub enum BindExecutorError {
    #[error("data sign or Decrypt fail.")]
    DisSign(#[from] ras_error::Error),
    #[error("data send fail.")]
    DisSend(#[from] ClientSendRequestError),
    #[error("data serializing fail.")]
    DisSer(#[from] serde_json_error::Error),
}
