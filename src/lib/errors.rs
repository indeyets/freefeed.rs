use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum FreefeedApiError {
    #[error("Authorization is required")]
    AuthorizationRequired,
    #[error("Resource was not found")]
    ResourceNotFound,
    #[error("Unknown API-access error")]
    UnknownNetworkError,
    #[error("Unknown API-response parsing error")]
    UnknownParseError,
    #[error("Unknown API error")]
    UnknownError,
}
