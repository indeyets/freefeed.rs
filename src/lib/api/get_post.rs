use serde_derive::{Deserialize};

use crate::errors::FreefeedApiError;
use crate::api::client::ApiClient;
use crate::api::data_structs::{Post, User, Attachment};
use chrono::{DateTime, Local, TimeZone};
use crate::api::Comment;

#[derive(Debug, Deserialize)]
struct PostsResponse {
    attachments: Vec<PostsResponseAttachment>,
    comments: Vec<PostsResponseComment>,
    posts: PostsResponsePost,
    users: Vec<PostsResponseUser>,
}

#[derive(Debug, Deserialize)]
pub struct PostsResponsePost {
    attachments: Vec<String>,
    body: String,
    #[serde(rename = "createdBy")]
    created_by: String,
    #[serde(rename = "createdAt")]
    created_at: String,
    #[serde(rename = "updatedAt")]
    updated_at: String,
    comments: Vec<String>,
    likes: Vec<String>,
}

#[derive(Clone, Debug, Deserialize)]
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

#[derive(Debug, Deserialize)]
pub struct PostsResponseComment {
    body: String,
    #[serde(rename = "createdBy")]
    created_by: String,
    #[serde(rename = "createdAt")]
    created_at: String,
    #[serde(rename = "updatedAt")]
    updated_at: String,
}

fn user_by_uuid(users: &Vec<PostsResponseUser>, uuid: &str) -> Option<PostsResponseUser> {
    let mut author: Option<&PostsResponseUser> = None;

    for user in users {
        if user.id == uuid {
            author = Some(user);
        }
    }

    author.cloned()
}

impl ApiClient {
    pub async fn get_a_post(self, post_uuid: &str) -> Result<Post, FreefeedApiError> {
        let path = format!("/v2/posts/{}?maxComments=all&maxLikes=", post_uuid);

        match self.get_from_api(&path).await {
            Ok(response) => match response.json::<PostsResponse>().await {
                Ok(response_struct) => {
                    let author_uuid: &str = &response_struct.posts.created_by;
                    let created_at: DateTime<Local> = Local.timestamp_millis(response_struct.posts.created_at.parse::<i64>().unwrap());
                    let updated_at: DateTime<Local> = Local.timestamp_millis(response_struct.posts.updated_at.parse::<i64>().unwrap());

                    let author = user_by_uuid(&response_struct.users, author_uuid);

                    let attachments = response_struct.attachments
                        .iter().map(|att| Attachment {
                            id: att.id.clone(),
                            file_name: att.file_name.clone(),
                            file_size: att.file_size.parse::<u64>().unwrap(),
                            url: att.url.clone(),
                        })
                        .collect();

                    let comments = response_struct.comments
                        .iter().map(|comment| Comment {
                            body: comment.body.clone(),
                            created_at: Local.timestamp_millis(comment.created_at.parse::<i64>().unwrap()),
                            updated_at: Local.timestamp_millis(comment.updated_at.parse::<i64>().unwrap()),
                            author: {
                                let comment_author = user_by_uuid(&response_struct.users, &comment.created_by);
                                let the_author = comment_author.unwrap();

                                User {
                                    uuid: the_author.id,
                                    username: the_author.username,
                                    screen_name: the_author.screen_name,
                                }
                            }
                        })
                        .collect();



                    match author {
                        Some(author) => {
                            Ok(Post {
                                attachments,
                                comments,
                                author: User {
                                    uuid: author.id,
                                    username: author.username,
                                    screen_name: author.screen_name,
                                },
                                body: String::from(&response_struct.posts.body),
                                likes: response_struct.posts.likes.iter().map(|uuid| -> User {
                                    let u = user_by_uuid(&response_struct.users, &uuid).unwrap();
                                    User {
                                        uuid: u.id,
                                        username: u.username,
                                        screen_name: u.screen_name,
                                    }
                                }).collect(),
                                created_at,
                                updated_at,
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
