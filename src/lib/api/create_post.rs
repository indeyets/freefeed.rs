use serde_derive::{Deserialize, Serialize};

use crate::errors::FreefeedApiError;
use crate::api::client::ApiClient;

#[derive(Debug, Serialize, Deserialize)]
struct PostPost {
    body: String,
    attachments: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct PostMeta {
    #[serde(rename = "commentsDisabled")]
    comments_disabled: bool,
    feeds: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Post {
    post: PostPost,
    meta: PostMeta,
}

impl ApiClient {
    pub async fn create_a_post(self, body: &str, feed: &str) -> Result<String, FreefeedApiError> {
        let post_obj = Post {
            post: PostPost {
                body: body.to_string(),
                attachments: vec![],
            },
            meta: PostMeta {
                comments_disabled: true,
                feeds: vec![feed.to_string()]
            }
        };

        match self.post_to_api("/v1/posts", &post_obj).await {
            Ok(response) => match response.json::<serde_json::Value>().await {
                Ok(response_struct) => {
                    println!("{}", response_struct);
                    let body_string: String = response_struct["posts"]["id"].to_string();
                    Ok(body_string)
                },
                _ => Err(FreefeedApiError::UnknownParseError),
            },
            Err(e) => Err(e)
        }
    }
}
