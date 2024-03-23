use rocket;

mod chat_room;
mod handlers;
mod metrics;


#[rocket::main]
async fn main() {
    let prom = metrics::get_prometheus();

    let _ = rocket::build()
        .attach(prom.clone())
        .mount("/", rocket::routes![
            handlers::chat,
        ])
        .mount("/metrics", prom)
        .manage(chat_room::ChatRoom::default())
        .launch()
        .await;
}
