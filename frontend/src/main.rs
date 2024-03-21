use yew::prelude::*;
use yew_hooks::use_websocket;

use common::{WebSocketMessage, WebSocketMessageType};

use crate::message_list::MessageList;
use crate::users_list::UsersList;
use crate::input::Input;

mod message_list;
mod users_list;
mod input;


#[function_component]
fn App() -> Html {
    let messages_handle = use_state(Vec::default);
    let messages = (*messages_handle).clone();

    let users_handle = use_state(Vec::default);
    let users = (*users_handle).clone();

    let username_handle = use_state(String::default);
    let username = (*username_handle).clone();

    let ws = use_websocket("ws://127.0.0.1:8000".to_string());

    let mut cloned_messages = messages.clone();
    use_effect_with(ws.message.clone(), move |ws_msg| {
        if let Some(msg) = &**ws_msg {
            let websocket_message: WebSocketMessage = match serde_json::from_str(msg) {
                Ok(msg) => msg,
                Err(err) => {
                    // TODO: add logs
                    println!("Error while deserializing ws message {}", err);
                    return;
                }
            };
            match websocket_message.message_type {
                WebSocketMessageType::NewMessage | WebSocketMessageType::System => {
                    if let Some(msg) = websocket_message.message {
                        cloned_messages.push(msg);
                        messages_handle.set(cloned_messages);
                    } else {
                        // TODO: add logs
                        println!("Missing message payload");
                    }
                },
                WebSocketMessageType::UserList => {
                    if let Some(users) = websocket_message.users {
                        users_handle.set(users);
                    } else {
                        // TODO: add logs
                        println!("Missing users payload");
                    }
                },
                WebSocketMessageType::UsernameChange => {
                    if let Some(username) = websocket_message.username {
                        username_handle.set(username);
                    } else {
                        // TODO: add logs
                        println!("Missing username payload");
                        return; 
                    }
                },
            }
        }
    });

    let cloned_ws = ws.clone();
    let send_message_callback = Callback::from(
        move |msg: WebSocketMessage| {
            cloned_ws.send(msg.to_string());
        }
    );

    html! {
        <div class="content">
            <div class="chat-wrapper">
                <div class="users window">
                    <UsersList users={users} username={username}/>
                    <Input 
                        callback={send_message_callback.clone()}
                        message_type={WebSocketMessageType::UsernameChange}
                        wrapper_name="change-username-wrapper"
                        placeholder="Set nickname..."
                        button_text="Change"
                    />
                </div>
                <div class="chat window">
                    <MessageList messages={messages}/>
                    <Input 
                        callback={send_message_callback.clone()}
                        message_type={WebSocketMessageType::NewMessage}
                        wrapper_name="input-wrapper"
                        placeholder="Type message..."
                        button_text="Send"
                    />
                </div>
            </div>
        </div>
    }
}


fn main() {
    yew::Renderer::<App>::new().render();
}
