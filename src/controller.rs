use sysinfo::{System, SystemExt};
use std::collections::HashMap;
use std::fs::File;
use std::os::fd;
use std::thread;
use std::time::Duration;
use tempfile::tempfile;

use crate::specs::info::*;
use crate::specs::capacity::*;
use crate::specs::processes::*;
use crate::specs::network::*;
use crate::specs::io::*;
use crate::ui::*;
use slint::*;

struct Measurements {
    info: Info,
    capacity: Capacity,
    processes: Vec<Process>,
    network: Vec<Network>,
    io: Vec<I_O>
}

fn fetch_measurements(mut temp_file: &File) -> Measurements{
    let mut sys = System::new_all();
    sys.refresh_all();
    return Measurements {
        info: fetch_info(&sys),
        capacity: fetch_capacity(&sys),
        processes: fetch_processes(&sys),
        network: fetch_network(&sys),
        io: fetch_io(&temp_file)
    };
}

pub fn start_measurements(window_weak: Weak<MainWindow>) {
    let mut temp_file = tempfile().unwrap();
    thread::spawn(move || {
        loop {
            tokio::runtime::Runtime::new().unwrap().block_on(async {
                measure_system(window_weak.clone(), &temp_file).await;
            });
            thread::sleep(Duration::from_millis(1000));
        }
    });
}

async fn measure_system(window_weak: Weak<MainWindow>,mut temp_file: &File) {
    let measurements = fetch_measurements(temp_file);
    window_weak.upgrade_in_event_loop(move |win| {
        win.global::<NetworkAdapter>().set_interfaces(mutate_network(measurements.network));
        win.global::<ProcessesAdapter>().set_processes(mutate_processes(measurements.processes));
        
    }).unwrap();
}
