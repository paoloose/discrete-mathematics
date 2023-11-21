use warp::reply::Reply;

use crate::Clients;
use crate::ws;

pub fn ws_handler(ws: warp::ws::Ws, clients: Clients) -> impl Reply {
    ws.on_upgrade(move |socket| {
        ws::client_connection(socket, clients)
    })
}

pub fn not_found_handler() -> impl Reply {
    warp::reply::with_status(
        "Not Found",
        warp::http::StatusCode::NOT_FOUND
    )
}

pub fn health_handler() -> impl Reply {
    warp::http::StatusCode::OK
}
