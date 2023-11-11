use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct ItemInfo{
    name: String,
    path: String,
    size: u64,
}
impl ItemInfo {
    pub fn new(name: String, path: String, size: u64) -> ItemInfo {
        ItemInfo{
            name,
            path,
            size,
        }
    }
}