use std::rc::Rc;
use sysinfo::{NetworkExt, System, SystemExt};
use slint::*;
use crate::ui::*;

pub struct Network {
    name: String,
    received: u64,
    transmitted: u64
}

pub fn mutate_network(interfaces: Vec<Network>) -> ModelRc<NetworkInterface> {
    let intrf: VecModel<NetworkInterface> = VecModel::default();
    for interface in interfaces {
        intrf.push(NetworkInterface { 
            name: interface.name.into(), 
            received: interface.received as i32, 
            transmitted: interface.transmitted as i32, 
        });
    }
    return Rc::new(VecModel::from(intrf)).into();
}

pub fn fetch_network(sys: &System) -> Vec<Network> {
    let mut interfaces: Vec<Network> = Vec::new();
    for (interface_name, data) in sys.networks() {
        let int: Network = Network { 
            name:        interface_name.to_string(), 
            received:    data.received(), 
            transmitted: data.transmitted()
        };
        interfaces.push(int);
    }
    interfaces.sort_by(|a, b| b.name.partial_cmp(&a.name).unwrap());
    return interfaces;
}