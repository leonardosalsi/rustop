use std::rc::Rc;
use std::{collections::HashMap, fs::File};
use std::io::prelude::*;
use std::sync::Arc;
use slint::*;
use crate::ui::*;

pub struct Disk {
    name: String,
    mb_read: f64,
    mb_write: f64,
    part: Vec<Partition>
}

pub struct Partition {
    name: String,
    mb_read: f64,
    mb_write: f64,
}

pub struct DiskMonitor {
    pub prev: HashMap<String, Disk>,
}

pub fn mutate_io(disks: Vec<Disk>) -> ModelRc<DiskIO> {
    let dsks: VecModel<DiskIO> = VecModel::default();
    for disk in disks {
        let prts: VecModel<PartitionIO> = VecModel::default();
        for part in disk.part {
            prts.push(PartitionIO { 
                name: part.name.into(), 
                read: part.mb_read as f32, 
                write: part.mb_write as f32
            });
        }
        dsks.push(DiskIO { 
            name: disk.name.into(), 
            read: disk.mb_read as f32, 
            write: disk.mb_write as f32,
            part: Rc::new(VecModel::from(prts)).into()
        });
    }
    return Rc::new(VecModel::from(dsks)).into();
}

pub fn fetch_io(diskmonitor: Arc<std::sync::Mutex<DiskMonitor>>) -> Vec<Disk> {
    let mut disks: HashMap<String, Disk> = HashMap::new();
    let mut curr: HashMap<String, Disk> = HashMap::new();
    let mut io_data = String::new();
    let mut fd = File::open(&"/proc/diskstats").unwrap();
    let mut diskmonitor = diskmonitor.lock().unwrap();
    fd.read_to_string(&mut io_data).unwrap();
    
    let mut last_disk_name: Option<String> = None;
    
    for line in io_data.lines() {
        let fields = line.split_whitespace().collect::<Vec<_>>();
        if fields.len() < 14 {
            continue;
        }
        if fields[2].contains("loop") || fields[2].contains("ram") {
            continue;
        }

        if fields[1] == "0" {
            let ds = Disk {
                name: fields[2].to_string(),
                mb_read: fields[5].parse::<f64>().unwrap() / 2048.0,
                mb_write: fields[9].parse::<f64>().unwrap() / 2048.0,
                part: Vec::new()
            };
            last_disk_name = Some(fields[2].to_string());
            if diskmonitor.prev.contains_key(fields[2]) {
                let pds = diskmonitor.prev.get(fields[2]).unwrap();
                disks.insert(fields[2].to_owned(), Disk { 
                    name: pds.name.clone(), 
                    mb_read: ds.mb_read - pds.mb_read, 
                    mb_write: ds.mb_write - pds.mb_write,
                    part: Vec::new()
                });
            } else {
                disks.insert(fields[2].to_owned(), ds);
            }

        } else {
            if let Some(disk_name) = &last_disk_name {
                if let Some(disk) = disks.get_mut(disk_name) {
                    let part = Partition {
                        name: fields[2].to_string(),
                        mb_read: fields[5].parse::<f64>().unwrap() / 2048.0,
                        mb_write: fields[9].parse::<f64>().unwrap() / 2048.0,
                    };
                    disk.part.push(part);
                }
            }
        }
        curr.insert(fields[2].to_owned(), Disk {
            name: fields[2].to_string(),
            mb_read: fields[5].parse::<f64>().unwrap() / 2048.0,
            mb_write: fields[9].parse::<f64>().unwrap() / 2048.0,
            part: Vec::new()
        });
    }
    
    diskmonitor.prev = curr;
    drop(diskmonitor);
    let mut disk_vec: Vec<Disk> = disks.into_iter().map(|(_, v)| v).collect();
    disk_vec.sort_by(|a, b| b.name.partial_cmp(&a.name).unwrap());
    return disk_vec;
}

