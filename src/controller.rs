use sysinfo::{System, SystemExt};
use std::thread;
use std::time::Duration;

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
    network: Network,
    io: I_O
}

fn fetch_measurements() -> Measurements{
    let mut sys = System::new_all();
    sys.refresh_all();
    return Measurements {
        info: fetch_info(&sys),
        capacity: fetch_capacity(&sys),
        processes: fetch_processes(&sys),
        network: fetch_network(&sys),
        io: fetch_io(&sys)
    };
}

pub fn start_measurements(window_weak: Weak<MainWindow>) {
    thread::spawn(move || {
        loop {
            tokio::runtime::Runtime::new().unwrap().block_on(async {
                measure_system(window_weak.clone()).await;
            });
            thread::sleep(Duration::from_millis(1000));
        }
    });
}
async fn measure_system(window_weak: Weak<MainWindow>) {
    let measurements = fetch_measurements();
    show_processes(window_weak, measurements.processes);
}
