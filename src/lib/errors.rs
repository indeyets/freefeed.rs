use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum FreefeedApiError {
    #[error("Authorization is required")]
    AuthorizationRequired,
    #[error("Unknown API-access error")]
    UnknownNetworkError,
    #[error("Unknown API-response parsing error")]
    UnknownParseError,
    #[error("Unknown API error")]
    UnknownError,
}
