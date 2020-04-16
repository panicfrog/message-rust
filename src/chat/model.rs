use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum ChatMessageType {
    // chat message
    OneToOne(usize),
    RoomMessage(String),
    Broadcast,
    // action
    Join(String),
    // message  ack
    Ack,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ChatMessage {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<usize>,
    pub style: ChatMessageType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,
}

impl ChatMessage {
    pub fn ack(message_id: String) -> Self {
        ChatMessage {
            from: None,
            style: ChatMessageType::Ack,
            content: None,
            message_id: Some(message_id),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ChatNameMessageType {
    // chat message
    OneToOne(String),
    RoomMessage(String),
    Broadcast,
    // action
    Join(String),
    // message  ack
    Ack,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ChatNameMessage {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
    pub style: ChatNameMessageType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,
}

impl ChatNameMessage {
    pub fn ack(message_id: String) -> Self {
        ChatNameMessage {
            from: None,
            style: ChatNameMessageType::Ack,
            content: None,
            message_id: Some(message_id),
        }
    }
}