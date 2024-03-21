use web_sys::HtmlTextAreaElement;
use yew::prelude::*;

use common::{ChatMessage, WebSocketMessage, WebSocketMessageType};

#[derive(PartialEq, Properties)]
pub struct InputProps {
    pub callback: Callback<WebSocketMessage>,
    pub message_type: WebSocketMessageType,
    pub wrapper_name: String,
    pub placeholder: String,
    pub button_text: String,
}

#[function_component(Input)]
pub fn get_input(props: &InputProps) -> Html {
    let InputProps { callback, message_type, wrapper_name, placeholder, button_text } = props;
    let new_value_handle = use_state(String::default);
    let new_value = (*new_value_handle).clone();

    let cloned_new_value_handle = new_value_handle.clone();
    let on_value_change = Callback::from(move |e: Event| {
        let target = e.target_dyn_into::<HtmlTextAreaElement>();
        if let Some(text_area) = target {
            cloned_new_value_handle.set(text_area.value());
        }
    });

    let cloned_new_value = new_value.clone();
    let cloned_message_type = message_type.clone();
    let callback = callback.clone();

    let on_button_click = Callback::from(move |_: MouseEvent| {
        if cloned_new_value.is_empty() {
            return;
        }
        let msg = match cloned_message_type {
            WebSocketMessageType::NewMessage => {
                WebSocketMessage::from_chat_msg(
                    ChatMessage::new(cloned_new_value.clone(), "".to_string())
                )
            },
            WebSocketMessageType::UsernameChange => {
                if cloned_new_value.to_lowercase() == "system" {
                    return;
                }
                WebSocketMessage::from_username(cloned_new_value.clone())
            },
            _ => {
                return;
            }

        };
        callback.emit(msg);
        new_value_handle.set("".to_string());
    });

    html! {
        <div class={wrapper_name}>
            <textarea
                type="text"
                placeholder={placeholder.to_owned()}
                class="text-input"
                value={new_value}
                onchange={on_value_change}
            ></textarea>
            <button type="submit" class="btn" onclick={on_button_click}>
                {button_text}
            </button>
        </div>
    }
}