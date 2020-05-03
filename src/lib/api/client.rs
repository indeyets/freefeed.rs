use reqwest::{Client, Error, Response, StatusCode};
use reqwest::header::{USER_AGENT};
use serde::Serialize;

use crate::errors::FreefeedApiError;
use crate::reqwest_extensions::Authorizable;

pub struct ApiClient {
    origin: String,
    token: Option<String>,
    reqwest: Client,
}

pub fn api_client(origin: &str, token: Option<&str>) -> ApiClient {
    return ApiClient {
        origin: String::from(origin),
        token: match token {
            Some(token) => Some(String::from(token)),
            None => None,
        },
        reqwest: Client::new(),
    };
}

fn handle_reqwest_errors(response: Result<Response, Error>) -> Result<Response, FreefeedApiError> {
    match response {
        Ok(response) => {
            let code = response.status();

            match code {
                StatusCode::OK => Ok(response),
                StatusCode::FORBIDDEN => Err(FreefeedApiError::AuthorizationRequired),
                StatusCode::UNAUTHORIZED => Err(FreefeedApiError::AuthorizationRequired),
                StatusCode::NOT_FOUND => Err(FreefeedApiError::ResourceNotFound),
                _ => Err(FreefeedApiError::UnknownError)
            }
        },
        _ => Err(FreefeedApiError::UnknownNetworkError)
    }
}

impl ApiClient {
    pub async fn get_from_api(self, path: &str) -> Result<Response, FreefeedApiError> {
        let url = format!("{}{}", self.origin, path);

        let request_builder = self.reqwest
            .get(&url)
            .header(USER_AGENT, "freefeed.rs 1.0")
            .authorize_maybe(self.token.as_deref());

        let response = request_builder.send().await;
        handle_reqwest_errors(response)
    }

    pub async fn post_to_api<T: Serialize + ?Sized>(self, path: &str, json: &T) -> Result<Response, FreefeedApiError> {
        let url = format!("{}{}", self.origin, path);

        let request_builder = self.reqwest
            .post(&url)
            .json(json)
            .header(USER_AGENT, "freefeed.rs 1.0")
            .authorize_maybe(self.token.as_deref());

        let response = request_builder.send().await;
        handle_reqwest_errors(response)
    }

    pub async fn put_to_api<T: Serialize + ?Sized>(self, path: &str, json: &T) -> Result<Response, FreefeedApiError> {
        let url = format!("{}{}", self.origin, path);

        let request_builder = self.reqwest
            .put(&url)
            .json(json)
            .header(USER_AGENT, "freefeed.rs 1.0")
            .authorize_maybe(self.token.as_deref());

        let response = request_builder.send().await;
        handle_reqwest_errors(response)
    }
}