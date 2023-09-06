
use std::{collections::HashMap, fs::File};
use std::io::{self, Write, prelude::*, SeekFrom};

use sysinfo::{ProcessExt, System, SystemExt, UserExt, PidExt};
use serde::{Serialize, Deserialize};
use serde_json::from_reader;

#[derive(Serialize, Deserialize)]
pub struct I_O {
    name: String,
    mb_read: f64,
    mb_write: f64
}

#[derive(Serialize, Deserialize)]
struct IOMAP(HashMap<String, I_O>);

fn load_map(mut temp_file: &File) -> HashMap<String, I_O> {
    let mut contents = String::new();
    temp_file.read_to_string(&mut contents).unwrap();

    let deserialized: IOMAP = serde_json::from_str(&contents).unwrap();
    let map = deserialized.0;

    return map;
}

fn write_map(mut temp_file: &File, map: HashMap<String, I_O>) {
    let serialized = serde_json::to_string(&IOMAP(map)).unwrap();
    let mut file = File::create("map.json").unwrap();
    file.write_all(serialized.as_bytes()).unwrap();
}


pub fn fetch_io(mut temp_file: &File) -> Vec<I_O> {
    let mut disks: Vec<I_O> = Vec::new();
    let mut curr: HashMap<String, I_O>= HashMap::new();
    let mut prev: HashMap<String, I_O> = load_map(&temp_file);
    let mut io_data = String::new();
    let mut fd = File::open(&"/proc/diskstats").unwrap();
    fd.read_to_string(&mut io_data).unwrap();
    for line in io_data.lines() {
        let fields = line.split_whitespace().collect::<Vec<_>>();
        if fields.len() < 14 {
            continue;
        }
        let ds = I_O {
            name: fields[2].to_string(),
            mb_read: fields[5].parse::<f64>().unwrap() / 2048.0,
            mb_write: fields[9].parse::<f64>().unwrap() / 2048.0,
        };
        if prev.contains_key(fields[2]) {
            let pds = prev.get(fields[2]).unwrap();
            let mb_read_s = ds.mb_read - pds.mb_read;
            let mb_wrtn_s = ds.mb_write - pds.mb_write;
            let dsk = I_O { name: fields[2].to_owned(), mb_read: mb_read_s, mb_write: mb_wrtn_s };
            disks.push(dsk);

            curr.insert(fields[2].to_owned(), ds);
        } else {
            let dsk = I_O { name: fields[2].to_owned(), mb_read: 0.00, mb_write: 0.00 };
            disks.push(dsk);
        }
        
    }
    write_map(temp_file, curr);
    print_disks(&disks);
    return disks;
}

pub fn print_disks(disks: &Vec<I_O>) {
    for disk in disks {
        println!("{}: {} MB/s read, {} MB/s write", disk.name, disk.mb_read, disk.mb_write);
    }
}