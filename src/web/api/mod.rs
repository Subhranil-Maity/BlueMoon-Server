use axum::{routing::{Route, get}, Router, response::{IntoResponse, Html}};
mod auth;
pub fn api_routes() -> Router{
    Router::new().route("/", get(hello))
    .nest("/auth",auth::auth_route())
}
async fn hello() -> impl IntoResponse {
    Html("Hel")
}