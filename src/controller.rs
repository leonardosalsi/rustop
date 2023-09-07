use sysinfo::{System, SystemExt};
use std::collections::HashMap;
use std::thread;
use std::time::Duration;
use std::sync::{Arc, Mutex};
use crate::specs::info::*;
use crate::specs::cpu::*;
use crate::specs::processes::*;
use crate::specs::network::*;
use crate::specs::io::*;
use crate::ui::*;
use slint::*;

struct Measurements {
    info: Info,
    cpu: Vec<CPU>,
    processes: Vec<Process>,
    network: Vec<Network>,
    io: Vec<I_O>
}

fn fetch_measurements(sys:Arc<std::sync::Mutex<System>>, diskmonitor: Arc<std::sync::Mutex<DiskMonitor>>) -> Measurements {
    let mut sys = sys.lock().unwrap();
    sys.refresh_all();
    return Measurements {
        info: fetch_info(&sys),
        processes: fetch_processes(&sys),
        network: fetch_network(&sys),
        io: fetch_io(diskmonitor),
        cpu: fetch_cpu(&sys)
    };
}

pub fn start_measurements(window_weak: Weak<MainWindow>) {
    let diskmonitor = Arc::new(Mutex::new(DiskMonitor {
        prev: HashMap::new(),
    }));
    let sys = Arc::new(Mutex::new(System::new_all()));
    thread::spawn(move || {
        let diskmonitor = Arc::clone(&diskmonitor);
        let sys = Arc::clone(&sys);
        loop {
            tokio::runtime::Runtime::new().unwrap().block_on(async {
                measure_system(window_weak.clone(), sys.clone(), diskmonitor.clone()).await;
            });
            thread::sleep(Duration::from_millis(1000));
        }
    });
}

async fn measure_system(window_weak: Weak<MainWindow>, sys:Arc<std::sync::Mutex<System>>, diskmonitor: Arc<std::sync::Mutex<DiskMonitor>>) {
    let measurements = fetch_measurements(sys, diskmonitor);
    window_weak.upgrade_in_event_loop(move |win| {
        win.global::<NetworkAdapter>().set_interfaces(mutate_network(measurements.network));
        win.global::<ProcessesAdapter>().set_processes(mutate_processes(measurements.processes));
        win.global::<IOAdapter>().set_disks(mutate_io(measurements.io));
        win.global::<InfoAdapter>().set_info(mutate_info(measurements.info));
        win.global::<CPUAdapter>().set_cpus(mutate_cpu(measurements.cpu));
    }).unwrap();
}
