use reqwest::{RequestBuilder};
use reqwest::header::{AUTHORIZATION};

pub trait Authorizable {
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
