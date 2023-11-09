use axum::{Router, response::IntoResponse, Json, routing::{get, post}};
use serde_json::{Value, json};

use crate::{auth::LoginPaylaod, utils::error::{Result, Error}};

pub fn auth_route() -> Router{
    Router::new().route("/login", post(api_login))
}

async fn api_login(payload: Json<LoginPaylaod>) -> Result<Json<Value>>{
    
    if payload.userid != "admin" || payload.pwd != "admin" {
        return Err(Error::LoginFail);
    }
    let body = Json(json!(
        {
            "status": "Success"
        }
    ));
    Ok(body)
}