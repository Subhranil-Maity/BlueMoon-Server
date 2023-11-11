use sysinfo::System;
use sysinfo::SystemExt;



static mut SYSTEM: Option<System> = None;
pub fn get_system() -> &'static mut System {
    let f = unsafe { SYSTEM.get_or_insert(System::new()) };
    f
}