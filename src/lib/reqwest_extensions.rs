use reqwest::{Client, Error, Response, RequestBuilder, StatusCode};
use reqwest::header::{AUTHORIZATION, USER_AGENT};
use serde::Serialize;

use crate::errors::FreefeedApiError;

trait Authorizable {
    fn authorize(self, token: &str) -> RequestBuilder;
    fn authorize_maybe(self, token: Option<&str>) -> RequestBuilder;
}

impl Authorizable for RequestBuilder {
    fn authorize(self, token: &str) -> RequestBuilder {
        self.header(AUTHORIZATION, format!("Bearer {}", token))
    }

    fn authorize_maybe(self, token: Option<&str>) -> RequestBuilder {
        match token {
            Some(token) => self.authorize(token),
            None => self
        }
    }
}

fn handle_reqwest_errors(response: Result<Response, Error>) -> Result<Response, FreefeedApiError> {
    match response {
        Ok(response) => {
            let code = response.status();

            println!("code = {}", code);
            match code {
                StatusCode::OK => Ok(response),
                StatusCode::FORBIDDEN => Err(FreefeedApiError::AuthorizationRequired),
                _ => Err(FreefeedApiError::UnknownError)
            }
        },
        _ => Err(FreefeedApiError::UnknownNetworkError)
    }
}

static ORIGIN: &str = "https://candy.freefeed.net";

pub fn api_client() -> Client {
    Client::new()
}

pub async fn get_from_api(path: &str, token: Option<&str>) -> Result<Response, FreefeedApiError> {
    let url = format!("{}{}", ORIGIN, path);

    let request_builder = api_client()
        .get(&url)
        .header(USER_AGENT, "freefeed.rs 1.0")
        .authorize_maybe(token);

    let response = request_builder.send().await;
    handle_reqwest_errors(response)
}

pub async fn post_to_api<T: Serialize + ?Sized>(path: &str, token: Option<&str>, json: &T) -> Result<Response, FreefeedApiError> {
    let url = format!("{}{}", ORIGIN, path);

    let request_builder = api_client()
        .post(&url)
        .json(json)
        .header(USER_AGENT, "freefeed.rs 1.0")
        .authorize_maybe(token);

    let response = request_builder.send().await;
    handle_reqwest_errors(response)
}

pub async fn put_to_api<T: Serialize + ?Sized>(path: &str, token: Option<&str>, json: &T) -> Result<Response, FreefeedApiError> {
    let url = format!("{}{}", ORIGIN, path);

    let request_builder = api_client()
        .put(&url)
        .json(json)
        .header(USER_AGENT, "freefeed.rs 1.0")
        .authorize_maybe(token);

    let response = request_builder.send().await;
    handle_reqwest_errors(response)
}