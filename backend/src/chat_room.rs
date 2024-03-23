
use std::collections::HashMap;

use rocket::{
    futures::{stream::SplitSink, SinkExt}, 
    tokio::sync::Mutex,
};
use rocket_ws::{Message, stream::DuplexStream};
use log;

use common::{WebSocketMessage, WebSocketMessageType};


pub enum UserStatus {
    Join,
    Left,
}

pub struct ChatRoomConnection {
    pub username: String,
    pub sink: SplitSink<DuplexStream, Message>,
}

impl ChatRoomConnection {
    fn new(username: String, sink: SplitSink<DuplexStream, Message>) -> ChatRoomConnection {
        ChatRoomConnection {
            username,
            sink,
        }
    }
}

#[derive(Default)]
pub struct ChatRoom {
    pub connections: Mutex<HashMap<usize, ChatRoomConnection>>,
}

impl ChatRoom {
    pub async fn add(&self, user_id: usize, ws_sink: SplitSink<DuplexStream, Message>) {
        let username = {
            let mut conns = self.connections.lock().await;
            let username = format!("user #{}", user_id);
            let connection = ChatRoomConnection::new(username.clone(), ws_sink);
            conns.insert(user_id, connection);
            username
        };
        self.send_username(user_id).await;
        self.broadcast_users_list().await;
        self.update_status(username, UserStatus::Join).await;
    }

    pub fn parse_message(&self, msg: String) -> Option<WebSocketMessage> {
        let new_msg: WebSocketMessage = match serde_json::from_str(msg.as_str()) {
            Ok(new_msg) => new_msg,
            Err(_) => {
                log::warn!("Cannot deserialize json message");
                return None;
            }
        };
        Some(new_msg)
    }

    pub async fn update_status(&self, username: String, status: UserStatus) {
        let mut conns = self.connections.lock().await;

        let msg = match status {
            UserStatus::Join => format!("{} join the chat", username.clone()),
            UserStatus::Left => format!("{} left the chat", username),
        };
        let msg_out = WebSocketMessage::from_system_msg(msg).to_string();
        for (_, conn) in conns.iter_mut() {
            let _ = conn.sink.send(Message::Text(msg_out.clone())).await;
        }
    }

    pub async fn change_username(&self, user_id: usize, new_username: String) {
        let mut conns = self.connections.lock().await;
        let user_conn = match conns.get_mut(&user_id) {
            Some(conn) => conn,
            _ => {
                log::warn!("Cannot find a user");
                return;
            }
        };
        let old_username = user_conn.username.clone();
        user_conn.username = new_username.clone();
        
        let update_msg = WebSocketMessage::from_username(new_username.clone()).to_string();
        let _ = user_conn.sink.send(Message::Text(update_msg)).await;

        let users: Vec<String> = conns.iter().map(|(_, conn)| conn.username.clone()).collect();
        let users_list_msg = WebSocketMessage::from_users_list(users).to_string();
        let system_msg = format!("{} changed username to {}", old_username, new_username);
        let system_msg = WebSocketMessage::from_system_msg(system_msg).to_string();

        for (_, conn) in conns.iter_mut() {
            let _ = conn.sink.send(Message::Text(system_msg.clone())).await;
            let _ = conn.sink.send(Message::Text(users_list_msg.clone())).await;
        }
    }

    pub async fn send_username(&self, user_id: usize) {
        let mut conns = self.connections.lock().await;
        if let Some(user_conn) = conns.get_mut(&user_id) {
            let msg = WebSocketMessage::from_username(user_conn.username.clone()).to_string();
            let _ = user_conn.sink.send(Message::Text(msg)).await;
        } else {
            log::warn!("Cannot find a user {}", user_id);
        }
    }

    pub async fn broadcast(&self, msg: Message) {
        let mut conns = self.connections.lock().await;
        for (_, conn) in conns.iter_mut() {
            let _ = conn.sink.send(msg.clone()).await;
        } 
    }

    pub async fn broadcast_message(&self, msg: WebSocketMessage, user_id: usize) {
        let mut chat_msg = match msg.message {
            Some(msg) => msg,
            _ => {
                log::warn!("Message to broadcast from user {} is empty", user_id);
                return;
            }
        };

        let mut conns = self.connections.lock().await;
        if chat_msg.author.is_empty() {
            let user_conn = match conns.get(&user_id) {
                Some(conn) => conn,
                _ => {
                    log::warn!("Cannot find a user {}", user_id);
                    return;
                }
            };
            chat_msg.author = user_conn.username.clone();
        }

        let msg_out = WebSocketMessage::from_chat_msg(chat_msg).to_string();
        for (_id, conn) in conns.iter_mut() {
            let _ = conn.sink.send(Message::Text(msg_out.clone())).await;
        }
    }

    pub async fn broadcast_users_list(&self) {
        let mut conns = self.connections.lock().await;
        let users: Vec<String> = conns.iter().map(|(_, conn)| conn.username.clone()).collect();

        let msg_out = WebSocketMessage::from_users_list(users).to_string();
        for (_id, conn) in conns.iter_mut() {
            let _ = conn.sink.send(Message::Text(msg_out.clone())).await;
        }
    }

    pub async fn handle_chat_msg(&self, user_id: usize, msg: String) {
        if let Some(new_msg) = self.parse_message(msg.clone()) {
            match new_msg.message_type {
                WebSocketMessageType::NewMessage => {
                    self.broadcast_message(new_msg, user_id).await;
                },
                WebSocketMessageType::UsernameChange => {
                    if let Some(new_username) = new_msg.username {
                        self.change_username(user_id, new_username.clone()).await;
                    } else {
                        log::warn!("New username is empty");
                    }

                },
                WebSocketMessageType::UserList => {
                    self.broadcast_users_list().await;
                },
                WebSocketMessageType::System => {
                    log::debug!("not implemented");
                },
            }
        } else {
            self.broadcast(Message::Text(msg)).await;
        }
    }

    pub async fn flush(&self, user_id: usize) {
        let username = {
            let mut conns = self.connections.lock().await;
            let user_conn = match conns.remove(&user_id) {
                Some(conn) => conn,
                _ => {
                    log::warn!("Cannot find a user {} to remove", user_id);
                    return;
                }
            };
            user_conn.username
        };

        self.update_status(username, UserStatus::Left).await;
        self.broadcast_users_list().await;
    }
}
