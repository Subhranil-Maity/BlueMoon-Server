use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub struct ItemInfo {
    name: String,
    path: String,
    size: u64,
    is_symlink: bool,
}
impl ItemInfo {
    pub fn new(name: String, path: String, size: u64, is_symlink: bool) -> ItemInfo {
        ItemInfo {
            name,
            path,
            size,
            is_symlink,
        }
    }
}
