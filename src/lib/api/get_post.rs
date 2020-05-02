use crate::errors::FreefeedApiError;
use crate::api::client::ApiClient;

impl ApiClient {
    pub async fn get_a_post(self, post_uuid: &str) -> Result<String, FreefeedApiError> {
        let path = format!("/v2/posts/{}?maxComments=all&maxLikes=", post_uuid);

        match self.get_from_api(&path).await {
            Ok(response) => match response.json::<serde_json::Value>().await {
                Ok(response_struct) => {
                    let body_string: String = response_struct["posts"]["body"].to_string();
                    Ok(body_string)
                },
                _ => Err(FreefeedApiError::UnknownParseError),
            },
            Err(e) => Err(e)
        }
    }
}
