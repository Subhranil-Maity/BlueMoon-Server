use axum::{
    routing::get,
    Router,
};
mod auth;
mod home;
mod info;


mod dir;
mod rename;
mod get_item_info;
mod move_fie;
mod delete_file_or_folder;
mod mkdir;
mod touch;

pub fn api_routes() -> Router {
    Router::new()
        .route("/sys-info", get(info::info_route))
        .route("/get-info", get(get_item_info::get_item_info_route))
        .nest("/auth", auth::auth_route())
}
