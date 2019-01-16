use serde::{Deserializer,Deserialize};

#[derive(Deserialize,Debug,Clone)]
pub struct APIResponse {
    pub ok: bool,
    pub result: Option<Vec<APIResult>>,
}

#[derive(Deserialize,Debug,Clone)]
pub struct APIResult {
    pub message: Message,
    pub update_id: i32,
}

#[derive(Deserialize,Debug,Clone)]
pub struct Message {
    pub text: Option<String>,
    pub from:User,
    pub chat:Chat,
}

#[derive(Deserialize,Debug,Clone)]
pub struct User {
    pub id: i32,
    pub first_name:String,
    pub last_name:Option<String>,
    pub username:Option<String>,
}

#[derive(Deserialize,Debug,Clone)]
pub struct Chat {
    #[serde(deserialize_with = "de_from_int")]
    pub id: String,
    #[serde(rename = "type")]
    pub chat_type:String,
    pub title:Option<String>,
}

fn de_from_int<'de, D>(deserializer: D) -> Result<String, D::Error>
    where D: Deserializer<'de>{
    let integer = i64::deserialize(deserializer)?;
    Ok(integer.to_string())
}