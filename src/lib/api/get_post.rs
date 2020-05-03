use serde_derive::{Deserialize};

use crate::errors::FreefeedApiError;
use crate::api::client::ApiClient;
use crate::api::data_structs::{Post, User, Attachment};

#[derive(Debug, Deserialize)]
struct PostsResponse {
    attachments: Vec<PostsResponseAttachment>,
    posts: PostsResponsePost,
    users: Vec<PostsResponseUser>,
}

#[derive(Debug, Deserialize)]
pub struct PostsResponsePost {
    attachments: Vec<String>,
    body: String,
    #[serde(rename = "createdBy")]
    created_by: String,
    comments: Vec<String>,
    likes: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct PostsResponseUser {
    id: String,
    description: String,
    #[serde(rename = "screenName")]
    screen_name: String,
    username: String,
}

#[derive(Debug, Deserialize)]
pub struct PostsResponseAttachment {
    id: String,
    #[serde(rename = "fileName")]
    file_name: String,
    #[serde(rename = "fileSize")]
    file_size: String,
    url: String
}

impl ApiClient {
    pub async fn get_a_post(self, post_uuid: &str) -> Result<Post, FreefeedApiError> {
        let path = format!("/v2/posts/{}?maxComments=all&maxLikes=", post_uuid);

        match self.get_from_api(&path).await {
            Ok(response) => match response.json::<PostsResponse>().await {
                Ok(response_struct) => {
                    let body_string: String = response_struct.posts.body;
                    let author_uuid: String = response_struct.posts.created_by;

                    let mut author: Option<PostsResponseUser> = None;
                    for user in response_struct.users {
                        if user.id == author_uuid {
                            author = Some(user);
                        }
                    }

                    let attachments = response_struct.attachments.
                        iter().map(|att| Attachment {
                        id: att.id.clone(),
                        file_name: att.file_name.clone(),
                        file_size: att.file_size.parse::<u64>().unwrap(),
                        url: att.url.clone(),
                    }).collect();

                    match author {
                        Some(author) => {
                            Ok(Post {
                                attachments,
                                author: User {
                                    uuid: author_uuid,
                                    username: author.username,
                                    screen_name: author.screen_name,
                                },
                                body: body_string
                            })
                        },
                        None => {
                            Err(FreefeedApiError::UnknownParseError)
                        },
                    }
                },
                Err(_e) => {
                    Err(FreefeedApiError::UnknownParseError)
                },
            },
            Err(e) => Err(e)
        }
    }
}
