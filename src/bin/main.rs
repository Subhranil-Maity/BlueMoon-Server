use axum::{routing::get, Router};
use bluemoon_client::{log, web::handler};
// use tracing::{Level, info};
// use tracing_subscriber::FmtSubscriber;
use std::net::SocketAddr;

use log::info;

#[tokio::main]
async fn main() {
    // bluemoon_client::utils::test_sys();
    let app = Router::new().route("/", get(root)).merge(handler());

    info(format!("Starting server at {}:{}", "0.0.0.0", 3000));
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> &'static str {
    "Hello, World!"
}
