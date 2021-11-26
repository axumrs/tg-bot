use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Update {
    pub update_id: u64,
    pub message: Message,
}

#[derive(Deserialize, Debug)]
pub struct Message {
    pub message_id: u64,
}
