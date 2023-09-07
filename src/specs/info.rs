use sysinfo::{System, SystemExt};
use compound_duration::format_dhms;
use crate::ui::*;
pub struct Info {
    sysname: String,
    syskernel: String,
    osver: String,
    hostname: String,
    uptime: String,
    total_mem: u64,
    used_mem: u64,
    total_swap: u64,
    used_swap: u64,
}

pub fn mutate_info(info: Info) -> SystemInformation {
    return SystemInformation {
        sysname: info.sysname.into(),
        syskernel: info.syskernel.into(),
        osver: info.osver.into(),
        hostname: info.hostname.into(),
        uptime: info.uptime.into(),
        used_mem: (info.used_mem as f32 / info.total_mem as f32 * 100.0).into(),
        used_swap: (info.used_swap as f32 / info.total_swap as f32 * 100.0).into()
    };
}

pub fn fetch_info(sys: &System) -> Info {
    return Info {
        sysname: sys.name().unwrap().to_string(),
        syskernel: sys.kernel_version().unwrap().to_string(),
        osver: sys.os_version().unwrap().to_string(),
        hostname: sys.host_name().unwrap().to_string(),
        uptime: format_dhms(sys.uptime()),
        total_mem: sys.total_memory(),
        used_mem: sys.used_memory(),
        total_swap: sys.total_swap(),
        used_swap: sys.used_swap()
    };
}