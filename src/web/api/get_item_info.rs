use std::path::{PathBuf, Path};
use std::str::FromStr;

use axum::Json;
use axum::extract::Query;
use serde::{Deserialize, Serialize};

use crate::log::info;
use crate::utils::model::ItemInfo;

use crate::utils::error::{Error, Result};

#[derive(Debug, Deserialize, Serialize)]
pub struct InfoQueryParams{
    path: Option<String>
}

pub async fn get_item_info_route(Query(params): Query<InfoQueryParams>) -> Result<Json<ItemInfo>>{
    // if params.path.is_none() {
    //     return Err(Error::QueryParamMissingOrNotValied)
    // }
    let path = match params.path {
        Some(p) => p,
        None => return Err(Error::QueryParamMissingOrNotValied),
    };
    let path_obj = Path::new(&path);
    if !(path_obj.is_absolute() && path_obj.has_root()) {
        return Err(Error::NotAValiedPath);
    }

    // let f = PathBuf::from_str(&path).;
    // info(format!("{:?}", f).to_string());
    let info = ItemInfo::new("name".to_string(), "path".to_string(), 1000);
    Ok(Json(info).into())
}