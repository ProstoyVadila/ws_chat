use rocket;

mod chat_room;
mod handlers;


#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount("/", rocket::routes![
            handlers::chat
        ])
        .manage(chat_room::ChatRoom::default())
        .launch()
        .await;
}
