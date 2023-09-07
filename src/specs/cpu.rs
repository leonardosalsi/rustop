use sysinfo::{System, SystemExt, CpuExt};
use std::rc::Rc;
use slint::*;
use crate::ui::*;

pub struct CPU {
    brand: String,
    name: String,
    usage: i32,
    freq: i32
}

pub fn mutate_cpu(cpus: Vec<CPU>) -> ModelRc<CPUInfo> {
    let cps: VecModel<CPUInfo> = VecModel::default();
    for cpu in cpus {
        cps.push(CPUInfo { 
            brand: cpu.brand.into(),
            name: cpu.name.into(), 
            usage: cpu.usage, 
            freq: cpu.freq
        });
        
    }
    return Rc::new(VecModel::from(cps)).into();
}

pub fn fetch_cpu(sys: &System) -> Vec<CPU> {
    let mut cpus: Vec<CPU> = Vec::new();
    for cpu in sys.cpus() {
        //println!("{:?}", cpu);
        let cpob = CPU {
            brand: cpu.brand().to_string(),
            name: cpu.name().to_string(),
            usage: cpu.cpu_usage() as i32,
            freq: cpu.frequency() as i32
        };
        cpus.push(cpob);
    }
    cpus.sort_by(|a, b| a.name.partial_cmp(&b.name).unwrap());
    return cpus;
}