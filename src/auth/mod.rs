use axum::Json;
use serde::Deserialize;
use crate::utils::error::{Error, Result};
use serde_json::{Value, json};


#[derive(Debug, Deserialize)]
pub struct LoginPaylaod{
    pub userid: String,
    pub pwd: String
}