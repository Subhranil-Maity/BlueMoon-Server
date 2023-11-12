use std::path::Path;

use serde::{Deserialize, Serialize};

use super::{error::Error, fs::is_path_valid};
use crate::utils::error::Result;

#[derive(Debug, Deserialize, Serialize)]
pub struct QueryParams {
    path: Option<String>,
}

impl QueryParams {
    pub fn get_path(&self) -> Result<&Path> {
        match &self.path {
            Some(p) => {
                let path = Path::new(p);
                is_path_valid(path)?;
                return Ok(path);
            }
            None => return Err(Error::QueryParamMissingOrNotValid),
        }
    }
}
