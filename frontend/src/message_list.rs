use common::ChatMessage;
use yew::prelude::*;


#[derive(Properties, PartialEq)]
pub struct MessageListProps {
    pub messages: Vec<ChatMessage>
}

#[function_component(MessageList)]
pub fn get_message_list(props: &MessageListProps) -> Html {
    html! {
        <div class="messages">
            <h3>{"Messages"}</h3>
            <ul id="chat">
                {
                    props.messages.iter().map(|m| {
                        if m.author.to_lowercase() == "system" {
                            html! {
                                <li class="message-system">
                                    <p class="message-timestamp">{m.created_at.format("%Y-%m-%d %H:%M").to_string()}</p>
                                    <p class="message-author"><b>{m.author.clone()}</b></p>
                                    <p class="message-text">{m.message.clone()}</p>
                                </li>
                            }
                        } else {
                            html! {
                                <li class="message">
                                    <p class="message-timestamp">{m.created_at.format("%Y-%m-%d %H:%M").to_string()}</p>
                                    <p class="message-author"><b>{m.author.clone()}</b></p>
                                    <p class="message-text">{m.message.clone()}</p>
                                </li>
                            }
                        }
                    }).collect::<Html>()
                }
            </ul>
        </div>
    }
}
