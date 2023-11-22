mod handlers;
mod ws;
mod client;
mod utils;

use client::Clients;
use warp::Filter;

#[tokio::main]
async fn main() {
    let clients = Clients::default();
    let with_clients = warp::any().map(move || clients.clone());

    let logger = warp::log("server");
    let headers_logger = warp::log::custom(|info| {
        println!(
            "headers: {:?}",
            info.request_headers()
        );
    });

    let health = warp::path("health")
        .map(handlers::health_handler);

    let initiate = warp::path("ws")
        .and(warp::ws())
        .and(with_clients)
        .map(handlers::ws_handler);

    let not_found_route = warp::any()
        .map(handlers::not_found_handler);

    let routes = health
        .or(initiate)
        .or(not_found_route)
        .with(logger)
        .with(headers_logger)
        .with(warp::cors().allow_any_origin());

    println!("Server listening on port 3030");

    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
