use axum::Router;

use super::api::api_routes;
pub fn handler() -> Router {
    Router::new().nest("/api", api_routes())
}
