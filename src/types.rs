use serde::{Deserialize, Deserializer};
use serde_derive::Deserialize;
use std::collections::VecDeque;

pub enum UpdateType {
    Message,
    ChannelPost,
    UserJoined,
    UserLeft,
    UnknownUpdate,
}

#[derive(Deserialize, Debug, Clone)]
pub struct APIResponse<T> {
    pub ok: bool,
    #[serde(rename = "result")]
    pub body: Option<T>,
    pub description: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Update {
    pub message: Option<Message>,
    pub channel_post: Option<Message>,
    pub update_id: i32,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Message {
    pub text: Option<String>,
    pub from: Option<User>,
    pub chat: Chat,
    pub message_id: i32,
    pub new_chat_members: Option<VecDeque<User>>,
    pub left_chat_member: Option<User>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct User {
    #[serde(deserialize_with = "de_from_int")]
    pub id: String,
    pub first_name: String,
    pub last_name: Option<String>,
    pub username: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Chat {
    #[serde(deserialize_with = "de_from_int")]
    pub id: String,
    #[serde(rename = "type")]
    pub chat_type: String,
    pub title: Option<String>,
    pub description: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ChatMember {
    pub user: User,
    pub status: String,
}

fn de_from_int<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let integer = i64::deserialize(deserializer)?;
    Ok(integer.to_string())
}
