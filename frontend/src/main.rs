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
            let websocket_message: WebSocketMessage = serde_json::from_str(msg).unwrap();
            match websocket_message.message_type {
                WebSocketMessageType::NewMessage | WebSocketMessageType::System => {
                    let msg = websocket_message.message.expect("Missing message payload");
                    cloned_messages.push(msg);
                    messages_handle.set(cloned_messages);
                },
                WebSocketMessageType::UserList => {
                    let users = websocket_message.users.expect("Missing users payload");
                    users_handle.set(users);
                },
                WebSocketMessageType::UsernameChange => {
                    let username = websocket_message.username.expect("Missing username payload");
                    username_handle.set(username);
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
