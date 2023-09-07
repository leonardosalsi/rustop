use std::rc::Rc;
use sysinfo::{ProcessExt, System, SystemExt, UserExt, PidExt};
use slint::*;
use compound_duration::format_dhms;

static mut SORT: Sort = Sort::DesPid;

pub fn sort_desc(col: i32) {
    match col{
        1=>unsafe { SORT = Sort::DesPid },
        2=>unsafe { SORT = Sort::DesUser },
        3=>unsafe { SORT = Sort::DesVirt },
        4=>unsafe { SORT = Sort::DesStatus },
        5=>unsafe { SORT = Sort::DesCpu },
        6=>unsafe { SORT = Sort::DesMem },
        7=>unsafe { SORT = Sort::DesTime },
        8=>unsafe { SORT = Sort::DesCommand },
        _=>unsafe { SORT = Sort::DesPid },
      }
} 

pub fn sort_asc(col: i32) {
    match col{
        1=>unsafe { SORT = Sort::AsPid },
        2=>unsafe { SORT = Sort::AsUser },
        3=>unsafe { SORT = Sort::AsVirt },
        4=>unsafe { SORT = Sort::AsStatus },
        5=>unsafe { SORT = Sort::AsCpu },
        6=>unsafe { SORT = Sort::AsMem },
        7=>unsafe { SORT = Sort::AsTime },
        8=>unsafe { SORT = Sort::AsCommand },
        _=>unsafe { SORT = Sort::AsPid },
      }
} 


enum Sort {
    AsPid,
    DesPid,
    AsUser,
    DesUser,
    AsVirt,
    DesVirt,
    AsStatus,
    DesStatus,
    AsCpu,
    DesCpu,
    AsMem,
    DesMem,
    AsTime,
    DesTime,
    AsCommand,
    DesCommand
}

pub struct Process {
    pid:     u32,
    user:    String,
    virt:    u64,
    status:  String,
    cpu:     f32,
    mem:     f32,
    time:    u64,
    command: String
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
        prc.push(to_standard_list_view_item(process.virt.to_string()));
        prc.push(to_standard_list_view_item(process.status));
        prc.push(to_standard_list_view_item(process.cpu.to_string()));
        prc.push(to_standard_list_view_item(process.mem.to_string()));
        prc.push(to_standard_list_view_item(format_dhms(process.time)));
        prc.push(to_standard_list_view_item(process.command));
        prcs.push(Rc::new(VecModel::from(prc)).into());
    }
    return Rc::new(VecModel::from(prcs)).into();
}

pub fn fetch_processes(sys: &System) -> Vec<Process> {
    let mut processes: Vec<Process> = Vec::new();
    for (pid, process) in sys.processes() {
        let prc = Process {
            pid:     pid.as_u32(),
            user:    sys.get_user_by_id(process.user_id().unwrap()).unwrap().name().to_string(),
            virt:    process.virtual_memory(),
            status:  process.status().to_string(),
            cpu:     process.cpu_usage(),
            mem:     (process.memory() as f32 / sys.total_memory() as f32 * 100.0),
            time:    process.run_time(),
            command: process.name().to_string(),
        };
        processes.push(prc);
    }
    
    unsafe {
        match &SORT {
            Sort::AsPid=>processes.sort_by(|a, b| a.pid.partial_cmp(&b.pid).unwrap()),
            Sort::DesPid=>processes.sort_by(|a, b| b.pid.partial_cmp(&a.pid).unwrap()),
            Sort::AsUser=>processes.sort_by(|a, b| a.user.partial_cmp(&b.user).unwrap()),
            Sort::DesUser=>processes.sort_by(|a, b| b.user.partial_cmp(&a.user).unwrap()),
            Sort::AsVirt=>processes.sort_by(|a, b| a.virt.partial_cmp(&b.virt).unwrap()),
            Sort::DesVirt=>processes.sort_by(|a, b| b.virt.partial_cmp(&a.virt).unwrap()),
            Sort::AsStatus=>processes.sort_by(|a, b| a.status.partial_cmp(&b.status).unwrap()),
            Sort::DesStatus=>processes.sort_by(|a, b| b.status.partial_cmp(&a.status).unwrap()),
            Sort::AsCpu=>processes.sort_by(|a, b| a.cpu.partial_cmp(&b.cpu).unwrap()),
            Sort::DesCpu=>processes.sort_by(|a, b| b.cpu.partial_cmp(&a.cpu).unwrap()),
            Sort::AsMem=>processes.sort_by(|a, b| a.mem.partial_cmp(&b.mem).unwrap()),
            Sort::DesMem=>processes.sort_by(|a, b| b.mem.partial_cmp(&a.mem).unwrap()),
            Sort::AsTime=>processes.sort_by(|a, b| a.time.partial_cmp(&b.time).unwrap()),
            Sort::DesTime=>processes.sort_by(|a, b| b.time.partial_cmp(&a.time).unwrap()),
            Sort::AsCommand=>processes.sort_by(|a, b| a.command.partial_cmp(&b.command).unwrap()),
            Sort::DesCommand=>processes.sort_by(|a, b| b.command.partial_cmp(&a.command).unwrap()),
        }
    }
    
    return processes;
}