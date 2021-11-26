use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct TextMessage {
    pub chat_id: u64,
    pub text: String,
}
