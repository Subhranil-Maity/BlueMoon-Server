use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct LoginPaylaod {
    pub userid: String,
    pub pwd: String,
}
