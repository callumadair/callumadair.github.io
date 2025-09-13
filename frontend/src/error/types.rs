use std::sync::Arc;

pub type Result<T> = core::result::Result<T, FrontendError>;

#[derive(Clone, Default, Debug, thiserror::Error)]
pub enum FrontendError
{
    #[error("{0}")]
    ReqwestError(#[from] Arc<reqwest::Error>),
    #[error("Unknown error encountered.")]
    #[default]
    UnknownError,
}

impl From<reqwest::Error> for FrontendError
{
    fn from(value: reqwest::Error) -> Self { Self::ReqwestError(Arc::new(value)) }
}
