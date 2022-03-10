use thiserror::Error;

#[derive(Error, Debug)]
pub(crate) enum TransError {
    // #[error("data store disconnected")]
    // Disconnect(#[from] std::io::Error),
    // #[error("the data for key `{0}` is not available")]
    // Redaction(String),
    // #[error("invalid header (expected {expected:?}, found {found:?})")]
    // InvalidHeader { expected: String, found: String },
    // #[error("unknown data store error")]
    // Unknown,
    #[error("Request must be json object")]
    RequestMustBeJsonObject,
    #[error("Request must contain `{0}`")]
    RequestMustContain(String),
    #[error("Request value of `{0}` must be string")]
    RequestValueMustBeString(String),
    #[error("crud_info for table_name[`{0}`] and crud_type[`{1}`] is not found")]
    CrudInfoNotFound(String, String),
}
