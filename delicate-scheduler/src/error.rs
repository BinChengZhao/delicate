use crate::prelude::*;
#[derive(ThisError, Debug)]
pub enum BindExecutorError {
    #[error("data sign fail.")]
    DisSign(#[from] ras_error::Error),
    #[error("data serializing fail.")]
    DisSer(#[from] serde_json_error::Error),
}
