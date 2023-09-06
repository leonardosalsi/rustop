use std::rc::Rc;
use sysinfo::{ProcessExt, System, SystemExt, UserExt, PidExt};
use slint::*;
use crate::ui::*;
use compound_duration::format_dhms;

pub struct Process {
    pid:     u32,
    user:    String,
    virt:    u64,
    status:  String,
    cpu:     f32,
    mem:     f32,
    time:    u64,
    command: String,
}

fn mutate_processes(processes : Vec<Process>) -> ModelRc<ModelRc<StandardListViewItem>> {
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
    processes.sort_by(|a, b| b.cpu.partial_cmp(&a.cpu).unwrap());
    return processes;
}

pub fn show_processes(window_weak: Weak<MainWindow>, processes : Vec<Process>) {
    window_weak.upgrade_in_event_loop(move |win| {
        win.global::<ProcessesAdapter>().set_processes(mutate_processes(processes));
    }).unwrap();
}

pub fn print_processes(processes : Vec<Process>) {
    for process in &processes {
        println!("{} {} {} {} {} {} {} {}", process.pid, process.user, process.virt, process.status, process.cpu, process.mem, process.time, process.command);
    }
}

/*
pub fn get_processes() -> ModelRc<ModelRc<StandardListViewItem>> {
    let processes: Vec<Process> = fetch_processes();
    return mutate_processes(processes);
}
*/