#![allow(dead_code)]

mod smartdevices;
mod smarthouse;
mod smartnet;

use core::panic;

use smartdevices::socket::SmartSocket;
use smartdevices::thermometer::SmartThermometer;
use smarthouse::room::SmartRoom;
use smarthouse::*;
use smartnet::SmartService;

use crate::smartdevices::SmartDevice;

fn main() {
    let mut socket1 = SmartDevice::from(SmartSocket::new("Socket1", "brief of SmartSocket1"));
    let mut socket2 = SmartDevice::from(SmartSocket::new("Socket2", "brief of SmartSocket2"));
    let mut socket3 = SmartDevice::from(SmartSocket::new("Socket3", "brief of SmartSocket3"));
    let mut thermo1 = SmartDevice::from(SmartThermometer::new("SmartThermometer1", -60.0, 60.0));

    let mut house = SmartHouse::new("Sweet Home");
    let mut room1 = SmartRoom::new("Room1");
    let mut room2 = SmartRoom::new("Room2");

    let ret = room1.add_device(&mut socket1);
    if ret.is_err() {
        dbg!(ret.unwrap_err());
        panic!()
    }
    let ret = room2.add_device(&mut socket2);
    if ret.is_err() {
        panic!("{}", ret.unwrap_err())
    }
    let ret = room2.add_device(&mut thermo1);
    if ret.is_err() {
        panic!("{}", ret.unwrap_err())
    }
    let ret = room2.add_device(&mut socket3);
    if ret.is_err() {
        panic!("{}", ret.unwrap_err())
    }

    let ret = house.add_room(room1);
    if ret.is_err() {
        panic!("{}", ret.unwrap_err())
    }

    let ret = house.add_room(room2);
    if ret.is_err() {
        panic!("{}", ret.unwrap_err())
    }

    let provider = OwningDeviceInfoProvider {
        devices: vec![socket1, socket2, socket3, thermo1],
    };

    let service = SmartService::new(house, provider, "127.0.0.1:8089".to_string());
    if service.is_err() {
        panic!("{}", service.unwrap_err())
    }
    service.unwrap().serve();
}
