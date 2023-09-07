use std::rc::Rc;
use std::{collections::HashMap, fs::File};
use std::io::prelude::*;
use std::sync::Arc;
use slint::*;
use crate::ui::*;

pub struct I_O {
    name: String,
    mb_read: f64,
    mb_write: f64
}

pub struct DiskMonitor {
    pub prev: HashMap<String, I_O>,
}

pub fn mutate_io(disks: Vec<I_O>) -> ModelRc<DiskIO> {
    let dsks: VecModel<DiskIO> = VecModel::default();
    for disk in disks {
        dsks.push(DiskIO { 
            name: disk.name.into(), 
            read: disk.mb_read as f32, 
            write: disk.mb_write as f32
        });
    }
    return Rc::new(VecModel::from(dsks)).into();
}

pub fn fetch_io(diskmonitor: Arc<std::sync::Mutex<DiskMonitor>>) -> Vec<I_O> {
    let mut disks: Vec<I_O> = Vec::new();
    let mut curr: HashMap<String, I_O> = HashMap::new();
    let mut io_data = String::new();
    let mut fd = File::open(&"/proc/diskstats").unwrap();
    let mut diskmonitor = diskmonitor.lock().unwrap();
    fd.read_to_string(&mut io_data).unwrap();

    for line in io_data.lines() {
        let fields = line.split_whitespace().collect::<Vec<_>>();
        if fields.len() < 14 {
            continue;
        }
        if fields[2].contains("loop") || fields[2].contains("ram") {
            continue;
        }
        let ds = I_O {
            name: fields[2].to_string(),
            mb_read: fields[5].parse::<f64>().unwrap() / 2048.0,
            mb_write: fields[9].parse::<f64>().unwrap() / 2048.0
        };
        unsafe {
            if diskmonitor.prev.contains_key(fields[2]) {
                let pds = diskmonitor.prev.get(fields[2]).unwrap();
                disks.push(I_O { 
                    name: pds.name.clone(), 
                    mb_read: ds.mb_read - pds.mb_read, 
                    mb_write: ds.mb_write - pds.mb_write 
                });
            }
        }
        curr.insert(fields[2].to_owned(), ds);
    }
    unsafe {
        diskmonitor.prev = curr;
    }
    drop(diskmonitor);
    disks.sort_by(|a, b| b.mb_read.partial_cmp(&a.mb_read).unwrap());
    print_disks(&disks);
    return disks;
}

pub fn print_disks(disks: &Vec<I_O>) {
    println!("Disks:");
    for disk in disks {
        println!("{}: {} MB/s read, {} MB/s write", disk.name, disk.mb_read, disk.mb_write);
    }
}