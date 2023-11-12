use axum::{routing::post, Json, Router};
use serde_json::{json, Value};

use crate::utils::{
    error::{Error, Result},
    LoginPaylaod,
};

pub async fn api_login(payload: Json<LoginPaylaod>) -> Result<Json<Value>> {
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
