use deno_core::error::JsError;
use tokio::task::JoinError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug, PartialEq)]
pub enum Error {
    #[error("execute script error: {0}")]
    ExecuteError(String),
    #[error("javascript error: {0}")]
    JsError(JsError),
    #[error("execute thread error: {0}")]
    TokioJoinError(String),
    #[error("invalid result: {0}")]
    InvalidResultError(String)
}

impl Error {
    pub fn from_deno_execute_script_error(err: anyhow::Error) -> Self {
        match err.downcast::<JsError>() {
            Ok(js_error) => Self::JsError(js_error),
            Err(err) => Self::ExecuteError(err.to_string()),
        }
    }
}

impl From<JoinError> for Error {
    fn from(err: JoinError) -> Self {
        Self::TokioJoinError(err.to_string())
    }
}
