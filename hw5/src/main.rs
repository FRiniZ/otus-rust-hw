#![allow(dead_code)]

mod integration_tests;

mod smartdevices;
mod smarthouse;

use core::panic;

use smartdevices::socket::SmartSocket;
use smartdevices::thermometer::SmartThermometer;
use smarthouse::room::SmartRoom;
use smarthouse::*;

use crate::smartdevices::SmartDevice;

fn main() {
    let mut socket1 = SmartSocket::new("SmartSocket1", "brief of SmartSocket1");

    let mut socket2 = SmartSocket::new("SmartSocket2", "brief of SmartSocket2");
    let mut socket3 = SmartSocket::new("SmartSocket3", "brief of SmartSocket3");

    let mut thermo = SmartThermometer::new("SmartThermometer1", -60.0, 60.0);

    let mut house = SmartHouse::new("Sweet Home");
    let mut room1 = SmartRoom::new("Room1");
    let mut room2 = SmartRoom::new("Room2");

    let ret = room1.add_device(SmartDevice::from(&mut socket1));
    if ret.is_err() {
        dbg!(ret.unwrap_err());
        panic!()
    }
    let ret = room2.add_device(SmartDevice::from(&mut socket2));
    if ret.is_err() {
        panic!("{}", ret.unwrap_err())
    }
    let ret = room2.add_device(SmartDevice::from(&mut thermo));
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

    let info_provider_1 = OwningDeviceInfoProvider {
        devices: vec![SmartDevice::from(&mut socket1)],
    };

    let report1 = house.create_report(&info_provider_1);

    let sd_socket2 = SmartDevice::from(&mut socket2);
    let sd_socket3 = SmartDevice::from(&mut socket3);
    let sd_thermo = SmartDevice::from(&mut thermo);

    let info_provider_2 = BorrowingDeviceInfoProvider {
        devices: vec![&sd_socket2, &sd_thermo, &sd_socket3],
    };

    let report2 = house.create_report(&info_provider_2);

    println!("Report #1: {report1}");
    println!("Report #2: {report2}");
}
