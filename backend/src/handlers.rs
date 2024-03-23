use std::sync::atomic::{AtomicUsize, Ordering};

use rocket::{
    futures::StreamExt, 
    State
};
use rocket_ws::{Channel, Message, WebSocket};
use log;

use crate::chat_room::ChatRoom;
use crate::metrics::{WS_NEW_CONNECTIONS_TOTAL, WS_CONNECTIONS_TOTAL};

static USER_ID_COUNTER: AtomicUsize = AtomicUsize::new(1);


#[rocket::get("/")]
pub fn chat<'r>(ws: WebSocket, state: &'r State<ChatRoom>) -> Channel<'r> {
    ws.channel(move |stream| Box::pin(async move {
        let user_id = USER_ID_COUNTER.fetch_add(1, Ordering::Relaxed);
        let (ws_sink, mut ws_stream) = stream.split();

        state.add(user_id, ws_sink).await;
        WS_NEW_CONNECTIONS_TOTAL.inc();
        WS_CONNECTIONS_TOTAL.inc();
    
        while let Some(msg) = ws_stream.next().await {
            if let Ok(msg_content) = msg {
                match msg_content {
                    Message::Text(json_msg) => {
                        state.handle_chat_msg(user_id, json_msg).await;
                    },
                    Message::Ping(_) => {},
                    Message::Pong(_) => {},
                    _ => {
                        // Unsupported
                        log::warn!("Unsupported message type {}", msg_content);

                    }
                }
            }
        }
        state.flush(user_id).await;
        WS_CONNECTIONS_TOTAL.dec();
    
        Ok(())
    }))
}
