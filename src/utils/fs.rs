use std::{fs, io};
use std::fs::Metadata;
use std::os::windows::fs::MetadataExt;
use std::path::Path;
use axum::response::IntoResponse;

use crate::utils::error::{Error, Result};
use crate::utils::model::ItemInfo;

pub fn is_path_valid(p: &Path) -> Result<()>{
    // let path_obj = Path::new(p);
    if !(p.is_absolute() && p.has_root()) {
        return Err(Error::NotAValidPath);
    }
    if !p.exists() {
        return Err(Error::FileOrFolderNotFound);
    }
    Ok(())
}

pub fn get_metadata(p: &Path) -> Result<Metadata>{
    is_path_valid(p)?;
    match fs::metadata(p) {
        Ok(metadata) => {return Ok(metadata);}
        Err(e) => {
            match e.kind() {
                io::ErrorKind::NotFound => Err(Error::FileOrFolderNotFound),
                io::ErrorKind::PermissionDenied => Err(Error::AccessDeniedBySystem),
                _ => Err(Error::UNKNOWN),
            }
        }
    }
}

pub fn get_file_info(path: &Path) -> ItemInfo{
    ItemInfo::new(
        path.file_name().unwrap_or_default().to_string_lossy().to_string(),
        get_cleaned_canonical_path(path),
        get_metadata(path).unwrap().file_size(),
        path.is_symlink()
    )
}
fn get_folder_name(path: &Path) -> Option<&str> {
    // let path = Path::new(folder_path);
    path.file_name().and_then(|name| name.to_str())
}

fn get_cleaned_canonical_path(path: &Path) -> String {
    let canonical_path = fs::canonicalize(path).unwrap().to_string_lossy().to_string();
    // Remove the \\?\ prefix from the canonical path
    if canonical_path.starts_with("\\\\?\\") {
        canonical_path[4..].to_string()
    } else {
        canonical_path
    }
}
pub fn get_folder_info(path: &Path) -> ItemInfo {
    ItemInfo::new(
        get_folder_name(path).unwrap_or_default().to_string(),
        get_cleaned_canonical_path(path),
        0,
        path.is_symlink()
    )
}

pub fn get_item_info(path: &Path) -> ItemInfo{
    if path.is_file() {
        return get_file_info(path);
    } else if path.is_dir(){
        return get_folder_info(path);
    }else if path.is_symlink() {
        return get_item_info(fs::canonicalize(path).unwrap().as_path());
    }
    else{
        return ItemInfo::default();
    }
}