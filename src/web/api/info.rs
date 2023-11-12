use axum::Json;
use serde::{Deserialize, Serialize};
use sysinfo::{Disk, DiskExt, DiskKind, SystemExt};

use crate::utils::bluenv::get_system;

#[derive(Deserialize, Serialize, Debug)]
pub struct Drive {
    disk_name: String,
    file_system_format: String,
    disk_type: String,
    removeable: bool,
    path: String,
    total_space: u64,
    used_space: u64,
}
impl Drive {
    pub fn new(disk: &Disk) -> Drive {
        Drive {
            disk_name: disk.name().to_string_lossy().to_string(),
            file_system_format: String::from_utf8(disk.file_system().to_vec())
                .unwrap_or("UNKNOWN".to_string()),
            disk_type: match disk.kind() {
                DiskKind::HDD => "HDD".to_string(),
                DiskKind::SSD => "SSD".to_string(),
                _ => "UNKNOWN".to_string(),
            },
            removeable: disk.is_removable(),
            path: disk.mount_point().to_str().unwrap().to_string(),
            total_space: disk.total_space(),
            used_space: disk.available_space(),
        }
    }
}

pub fn get_drive_arr() -> Vec<Drive> {
    let f = get_system();
    f.refresh_disks_list();
    let n = f.disks();
    let mut ret: Vec<Drive> = Vec::new();
    for m in n {
        ret.push(Drive::new(m));
    }
    ret
}

#[derive(Deserialize, Serialize)]
pub struct Info {
    device_name: String,
    platform: String,
    version: String,
    drive: Vec<Drive>,
}
impl Info {
    pub fn get() -> Info {
        let system = get_system();
        // system.refresh_system();

        Info {
            device_name: system.host_name().unwrap_or("".to_string()),
            platform: system.name().unwrap_or("".to_string()),
            version: system.os_version().unwrap_or("".to_string()),
            drive: get_drive_arr(),
        }
    }
}

pub async fn info_route() -> Json<Info> {
    Info::get().into()
}
