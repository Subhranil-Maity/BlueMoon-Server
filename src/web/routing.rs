use axum::{routing::get, Router, response::{IntoResponse, Html}};

use super::api::api_routes;
pub fn handler() -> Router {
    // let something = api_routes();
    Router::new()
    .nest("/api", api_routes())
    // .route("/hello", get(hello))
    
}

