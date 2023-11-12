use std::fs;
use std::path::Path;

use axum::Json;
use axum::extract::Query;

use crate::utils::model::ItemInfo;

use crate::utils::error::{Error, Result};
use crate::utils::fs::{get_item_info, get_metadata};
use crate::utils::query::QueryParams;

pub async fn get_item_info_route(Query(params): Query<QueryParams>) -> Result<Json<ItemInfo>> {
    let path = params.get_path()?;
    let metadata = get_metadata(path)?;
    let info = get_item_info(path);
    
    Ok(Json(info).into())
}
