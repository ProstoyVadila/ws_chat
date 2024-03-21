use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::json;


#[derive(Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum WebSocketMessageType {
    NewMessage,
    UserList,
    UsernameChange,
    System,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct WebSocketMessage {
    pub message_type: WebSocketMessageType,
    pub message: Option<ChatMessage>,
    pub users: Option<Vec<String>>,
    pub username: Option<String>,
}

impl WebSocketMessage {
    pub fn from_chat_msg(message: ChatMessage) -> WebSocketMessage {
        WebSocketMessage {
            message_type: WebSocketMessageType::NewMessage,
            message: Some(message),
            users: None,
            username: None,
        }
    }

    pub fn from_users_list(users: Vec<String>) -> WebSocketMessage {
        WebSocketMessage {
            message_type: WebSocketMessageType::UserList,
            message: None,
            users: Some(users),
            username: None,
        }
    }

    pub fn from_username(username: String) -> WebSocketMessage {
        WebSocketMessage {
            message_type: WebSocketMessageType::UsernameChange,
            message: None,
            users: None,
            username: Some(username),
        }
    }

    pub fn from_system_msg(message: String) -> WebSocketMessage {
        let message = ChatMessage::new(message, "system".to_string());
        WebSocketMessage {
            message_type: WebSocketMessageType::System,
            message: Some(message),
            users: None,
            username: None,
        }
    }
}

impl ToString for WebSocketMessage {
    fn to_string(&self) -> String {
        json!(self).to_string()
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct ChatMessage {
    pub message: String,
    pub author: String,
    pub created_at: NaiveDateTime,
}

impl ChatMessage {
    pub fn new(message: String, author: String) -> ChatMessage {
        ChatMessage {
            message,
            author,
            created_at: Utc::now().naive_utc(),
        }
    }
}

impl ToString for ChatMessage {
    fn to_string(&self) -> String {
        json!(self).to_string()
    }
}
