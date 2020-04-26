use crate::reqwest_extensions::{get_from_api};
use crate::errors::FreefeedApiError;

pub async fn get_a_post(post_uuid: &str, token: Option<&str>) -> Result<String, FreefeedApiError> {
    let path = format!("/v2/posts/{}?maxComments=all&maxLikes=", post_uuid);
    match get_from_api(&path, token).await {
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
