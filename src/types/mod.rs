use serde::Deserialize;
pub mod request;

#[derive(Deserialize, Debug)]
pub struct Update {
    pub update_id: u64,
    pub message: Message,
}

#[derive(Deserialize, Debug)]
pub struct Message {
    pub message_id: u64,
    pub from: User,
    pub chat: Chat,
    pub date: u64,
    pub text: String,
}

#[derive(Deserialize, Debug)]
pub struct User {
    pub id: u64,
    pub is_bot: bool,
    pub first_name: String,
    pub username: String,
    pub language_code: String,
}
#[derive(Deserialize, Debug)]
pub struct Chat {
    pub id: u64,
    pub first_name: String,
    pub username: String,
    #[serde(rename(deserialize = "type"))]
    pub types: String,
}
