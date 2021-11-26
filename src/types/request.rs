use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct TextMessage {
    pub chat_id: u64,
    pub text: String,
}
#[derive(Serialize, Debug)]
pub struct PhotoMessage {
    pub chat_id: u64,
    pub photo: String,
}
#[derive(Serialize, Debug)]
pub struct MarkdownMessage {
    pub chat_id: u64,
    pub text: String,
    pub parse_mode: String,
}
