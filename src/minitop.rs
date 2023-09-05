
use std::{thread, time::Duration};

use sysinfo::{NetworkExt, NetworksExt, ProcessExt, System, SystemExt, UserExt};
slint::include_modules!();
use compound_duration::format_dhms;

pub struct ProcessInfo {
    pid:     String,
    user:    String,
    virt:    String,
    status:  String,
    cpu:     String,
    mem:     String,
    time:    String,
    command: String,
}

pub struct Measurements {

}

pub fn setup() -> thread::JoinHandle<()> {

    thread::spawn(move || {
        loop {
            println!("Hello from minitop!");
            thread::sleep(Duration::from_secs(1));
        }
    })
}

async fn fetch_measurements() -> Measurements {
    let mut sys = System::new_all();
    sys.refresh_all();
    let mut measurements = Measurements {};
    return measurements;
}


fn fetch_processes() -> Vec<ProcessInfo> {
    let mut prcs = Vec::new();
    let mut sys = System::new_all();
    sys.refresh_all();
    for (pid, process) in sys.processes() {
        prcs.push(ProcessInfo {
            pid:     pid.to_string(),
            user:    sys.get_user_by_id(process.user_id().unwrap()).unwrap().name().to_string(),
            virt:    process.virtual_memory().to_string(),
            status:  process.status().to_string(),
            cpu:     process.cpu_usage().to_string(),
            mem:     (process.memory() / sys.total_memory() * 100).to_string(),
            time:    format_dhms(process.run_time()).to_string(),
            command: process.name().to_string(),
        });
    }
    return prcs;
}

fn print_processes(prcs: Vec<ProcessInfo>) {
    for prc in prcs {
        println!{"{} {} {} {} {} {} {} {}", prc.pid, prc.user, prc.virt, prc.status, prc.cpu, prc.mem, prc.time, prc.command};
    }
}

pub fn print() {
    let prcs = fetch_processes();
    print_processes(prcs);
}