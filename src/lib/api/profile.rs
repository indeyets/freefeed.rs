use serde_derive::{Deserialize, Serialize};

use crate::reqwest_extensions::{get_from_api, put_to_api};
use crate::errors::FreefeedApiError;

#[derive(Debug, Serialize)]
struct Profile<'l> {
    user: ProfileUser<'l>,
}

#[derive(Debug, Serialize)]
pub struct ProfileUser<'l> {
    #[serde(rename = "screenName", skip_serializing_if = "Option::is_none")]
    screen_name: Option<&'l str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<&'l str>,
}

pub fn make_profile_user<'l>() -> ProfileUser<'l> {
    return ProfileUser {
        screen_name: None,
        description: None
    };
}

#[derive(Debug, Deserialize)]
struct ProfileResponse {
    users: ProfileResponseUser,
}

#[derive(Debug, Deserialize)]
pub struct ProfileResponseUser {
    #[serde(rename = "screenName")]
    screen_name: String,
    description: String,
}

pub async fn get_me(token: &str) -> Result<String, FreefeedApiError> {
    let path = "/v1/users/me";
    match get_from_api(&path, Some(token)).await {
        Ok(response) => match response.json::<serde_json::Value>().await {
            Ok(response_struct) => {
                println!("{}", response_struct);
                let body_string: String = response_struct["users"]["username"].to_string();
                Ok(body_string)
            },
            _ => Err(FreefeedApiError::UnknownParseError),
        },
        Err(e) => Err(e)
    }
}

pub async fn change_profile(user_uuid: &str, user: ProfileUser<'_>, token: &str) -> Result<ProfileResponseUser, FreefeedApiError> {
    let path = format!("/v1/users/{}", user_uuid);
    let payload = Profile { user };

    match put_to_api(&path, Some(token), &payload).await {
        Ok(response) => match response.json::<ProfileResponse>().await {
            Ok(response_struct) => Ok(response_struct.users),
            _ => Err(FreefeedApiError::UnknownParseError),
        },
        Err(e) => Err(e)
    }
}

pub async fn change_screen_name(user_uuid: &str, screen_name: &str, token: &str) -> Result<String, FreefeedApiError> {
    let user = ProfileUser { screen_name: Some(screen_name), ..make_profile_user() };

    match change_profile(user_uuid, user, token).await {
        Ok(response_user) => Ok(response_user.screen_name),
        Err(e) => Err(e),
    }
}

pub async fn change_description(user_uuid: &str, description: &str, token: &str) -> Result<String, FreefeedApiError> {
    let user = ProfileUser { description: Some(description), ..make_profile_user() };

    match change_profile(user_uuid, user, token).await {
        Ok(response_user) => Ok(response_user.description),
        Err(e) => Err(e),
    }
}
