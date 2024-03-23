use rocket_prometheus::{self, prometheus::{IntCounter, IntGauge}, PrometheusMetrics};
use once_cell::sync::Lazy;
use log;


pub static WS_CONNECTIONS_TOTAL: Lazy<IntGauge> = Lazy::new(|| {
    IntGauge::new("ws_server_connections_total", "an amount of ws connections to the server")
        .expect("Cannot create ws_connections_amount metric")
});

pub static WS_NEW_CONNECTIONS_TOTAL: Lazy<IntCounter> = Lazy::new(|| {
    IntCounter::new("ws_server_new_connections_total", "a counter of all new connections to the server")
        .expect("Cannot create new_connections_counter metric")
});

pub fn get_prometheus() -> PrometheusMetrics {
    log::info!("Setting up prometheus metrics");
    let prom = PrometheusMetrics::new();
    prom.registry().register(Box::new(WS_CONNECTIONS_TOTAL.clone()))
        .expect("Cannot register ws connection amount metric");
    prom.registry().register(Box::new(WS_NEW_CONNECTIONS_TOTAL.clone()))
        .expect("Cannot register new_connections_counter metric");
    prom
}
