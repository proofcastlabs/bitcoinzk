use thiserror::Error;

#[derive(Debug, Error)]
pub(crate) enum Error {
    #[error("serde json error: {0}")]
    SerdeJson(#[from] serde_json::Error),
}
