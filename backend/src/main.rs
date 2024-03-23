use rocket;
use env_logger;

mod chat_room;
mod handlers;
mod metrics;


#[rocket::main]
async fn main() {
    env_logger::init();
    let prom = metrics::get_prometheus();

    log::info!("Starting ws server...");
    let _ = rocket::build()
        .attach(prom.clone())
        .mount("/", rocket::routes![
            handlers::chat,
        ])
        .mount("/metrics", prom)
        .manage(chat_room::ChatRoom::default())
        .launch()
        .await;

    log::info!("Ws server is stopped.")
}
