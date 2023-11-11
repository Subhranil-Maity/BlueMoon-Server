use axum::{routing::post, Router};

mod login;


pub fn auth_route() -> Router {
    Router::new().route("/login", post(login::api_login))
}
