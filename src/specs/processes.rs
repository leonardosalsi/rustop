use std::{rc::Rc, path::Path, fs::File, io::{BufReader, Read}};
use sysinfo::{ProcessExt, System, SystemExt, UserExt, PidExt};
use slint::*;
use compound_duration::format_dhms;

static mut SORT: Sort = Sort::DesPid;

pub fn sort_desc(col: i32) {
    match col{
        0=>unsafe { SORT = Sort::DesPid },
        1=>unsafe { SORT = Sort::DesUser },
        2=>unsafe { SORT = Sort::DesRes },
        3=>unsafe { SORT = Sort::DesSha },
        4=>unsafe { SORT = Sort::DesStatus },
        5=>unsafe { SORT = Sort::DesCpu },
        6=>unsafe { SORT = Sort::DesMem },
        7=>unsafe { SORT = Sort::DesRead },
        8=>unsafe { SORT = Sort::DesWrite },
        9=>unsafe { SORT = Sort::DesTime },
        10=>unsafe { SORT = Sort::DesCommand },
        _=>unsafe { SORT = Sort::DesPid },
      }
} 

pub fn sort_asc(col: i32) {
    match col{
        0=>unsafe { SORT = Sort::AsPid },
        1=>unsafe { SORT = Sort::AsUser },
        2=>unsafe { SORT = Sort::AsRes },
        3=>unsafe { SORT = Sort::AsSha },
        4=>unsafe { SORT = Sort::AsStatus },
        5=>unsafe { SORT = Sort::AsCpu },
        6=>unsafe { SORT = Sort::AsMem },
        7=>unsafe { SORT = Sort::AsRead },
        8=>unsafe { SORT = Sort::AsWrite },
        9=>unsafe { SORT = Sort::AsTime },
        10=>unsafe { SORT = Sort::AsCommand },
        _=>unsafe { SORT = Sort::AsPid },
      }
} 


enum Sort {
    AsPid,
    DesPid,
    AsUser,
    DesUser,
    AsRes,
    DesRes,
    AsSha,
    DesSha,
    AsStatus,
    DesStatus,
    AsCpu,
    DesCpu,
    AsMem,
    DesMem,
    AsRead,
    DesRead,
    AsWrite,
    DesWrite,
    AsTime,
    DesTime,
    AsCommand,
    DesCommand
}

pub struct Process {
    pid:     u32,
    user:    String,
    res:    u64,
    sha:   u64,
    status:  String,
    cpu:     f32,
    mem:     f32,
    read:    u64,
    write:   u64,
    time:    u64,
    command: String
}

fn read_memory_usage(pid: u32) -> (u64, u64) {
    let path = std::format!("/proc/{}/statm", pid);
    let file = File::open(Path::new(&path)).unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();

    buf_reader.read_to_string(&mut contents).unwrap();

    let parts: Vec<&str> = contents.split_whitespace().collect();
    if parts.len() < 2 {
        return (0, 0);
    }

    
    let resident_memory: u64 = parts[1].parse().unwrap_or(0);
    let shared_memory: u64 = parts[2].parse().unwrap_or(0);

    return (resident_memory, shared_memory);
}

fn convert_size(size: u64) -> String {
    const KILO: u64 = 1024;
    const MEGA: u64 = KILO * 1024;
    const GIGA: u64 = MEGA * 1024;
    const TERA: u64 = GIGA * 1024;

    if size < KILO {
        return std::format!("{} Bytes", size);
    } else if size < MEGA {
        return std::format!("{:.1} KB", (size as f64) / (KILO as f64));
    } else if size < GIGA {
        return std::format!("{:.1} MB", (size as f64) / (MEGA as f64));
    } else if size < TERA {
        return std::format!("{:.1} GB", (size as f64) / (GIGA as f64));
    } else {
        return std::format!("{:.1} TB", (size as f64) / (TERA as f64));
    }
}

pub fn mutate_processes(processes : Vec<Process>) -> ModelRc<ModelRc<StandardListViewItem>> {
    let prcs: VecModel<ModelRc<StandardListViewItem>> = VecModel::default();
    fn to_standard_list_view_item<T: Into<slint::SharedString>>(value: T) -> StandardListViewItem {
        let shared_string: slint::SharedString = value.into();
        return StandardListViewItem::from(shared_string).into();
    }
    for process in processes {

        let prc: VecModel<StandardListViewItem> = VecModel::default();
        prc.push(to_standard_list_view_item(process.pid.to_string()));
        prc.push(to_standard_list_view_item(process.user));
        prc.push(to_standard_list_view_item(convert_size(process.res)));
        prc.push(to_standard_list_view_item(convert_size(process.sha)));
        prc.push(to_standard_list_view_item(process.status));
        prc.push(to_standard_list_view_item(process.cpu.to_string()));
        prc.push(to_standard_list_view_item(process.mem.to_string()));
        prc.push(to_standard_list_view_item(convert_size(process.read)));
        prc.push(to_standard_list_view_item(convert_size(process.write)));
        prc.push(to_standard_list_view_item(format_dhms(process.time)));
        prc.push(to_standard_list_view_item(process.command));
        prcs.push(Rc::new(VecModel::from(prc)).into());
    }
    return Rc::new(VecModel::from(prcs)).into();
}

pub fn fetch_processes(sys: &System) -> Vec<Process> {
    let mut processes: Vec<Process> = Vec::new();
    for (pid, process) in sys.processes() {
        let (resmem, shamem) = read_memory_usage(pid.as_u32());
        let prc = Process {
            pid:     pid.as_u32(),
            user:    sys.get_user_by_id(process.user_id().unwrap()).unwrap().name().to_string(),
            res:     resmem,
            sha:     shamem,
            status:  process.status().to_string(),
            cpu:     process.cpu_usage() / sys.cpus().into_iter().count() as f32,
            mem:     (process.memory() as f32 / sys.total_memory() as f32 * 100.0),
            read:    process.disk_usage().read_bytes,
            write:   process.disk_usage().written_bytes,
            time:    process.run_time(),
            command: process.name().to_string(),
        };
        processes.push(prc);
    }
    
    unsafe {
        match &SORT {
            Sort::AsPid=>processes.sort_by(|a, b| a.pid.cmp(&b.pid)),
            Sort::DesPid=>processes.sort_by(|a, b| b.pid.cmp(&a.pid)),
            Sort::AsUser=>processes.sort_by(|a, b| a.user.cmp(&b.user)),
            Sort::DesUser=>processes.sort_by(|a, b| b.user.cmp(&a.user)),
            Sort::AsRes=>processes.sort_by(|a, b| a.res.cmp(&b.res)),
            Sort::DesRes=>processes.sort_by(|a, b| b.res.cmp(&a.res)),
            Sort::AsSha=>processes.sort_by(|a, b| a.sha.cmp(&b.sha)),
            Sort::DesSha=>processes.sort_by(|a, b| b.sha.cmp(&a.sha)),
            Sort::AsStatus=>processes.sort_by(|a, b| a.status.cmp(&b.status)),
            Sort::DesStatus=>processes.sort_by(|a, b| b.status.cmp(&a.status)),
            Sort::AsCpu=>processes.sort_by(|a, b| a.cpu.partial_cmp(&b.cpu).unwrap()),
            Sort::DesCpu=>processes.sort_by(|a, b| b.cpu.partial_cmp(&a.cpu).unwrap()),
            Sort::AsMem=>processes.sort_by(|a, b| a.mem.partial_cmp(&b.mem).unwrap()),
            Sort::DesMem=>processes.sort_by(|a, b| b.mem.partial_cmp(&a.mem).unwrap()),
            Sort::AsRead=>processes.sort_by(|a, b| a.read.cmp(&b.read)),
            Sort::DesRead=>processes.sort_by(|a, b| b.read.cmp(&a.read)),
            Sort::AsWrite=>processes.sort_by(|a, b| a.write.cmp(&b.write)),
            Sort::DesWrite=>processes.sort_by(|a, b| b.write.cmp(&a.write)),
            Sort::AsTime=>processes.sort_by(|a, b| a.time.cmp(&b.time)),
            Sort::DesTime=>processes.sort_by(|a, b| b.time.cmp(&a.time)),
            Sort::AsCommand=>processes.sort_by(|a, b| a.command.cmp(&b.command)),
            Sort::DesCommand=>processes.sort_by(|a, b| b.command.cmp(&a.command)),
        }
    }
    
    return processes;
}