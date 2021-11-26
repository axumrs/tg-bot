use serde::Deserialize;
pub mod request;

pub enum MsgType {
    Text(String),
    Photo(String),
    Markdown(String),
}

#[derive(Deserialize, Debug)]
pub struct Update {
    pub update_id: u64,
    pub message: Message,
}

#[derive(Deserialize, Debug)]
pub struct Message {
    pub message_id: u64,
    pub from: Option<User>,
    pub chat: Chat,
    pub date: u64,
    pub text: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct User {
    pub id: u64,
    pub is_bot: bool,
    pub first_name: Option<String>,
    pub username: Option<String>,
    pub language_code: Option<String>,
}
#[derive(Deserialize, Debug)]
pub struct Chat {
    pub id: u64,
    pub first_name: Option<String>,
    pub username: Option<String>,
    #[serde(rename(deserialize = "type"))]
    pub types: String,
}

#[derive(Deserialize, Debug)]
pub struct Response {
    pub ok: bool,
    pub result: Option<Message>,
}
