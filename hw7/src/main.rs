#![allow(dead_code)]

mod smartdevices;
mod smarthouse;

use core::panic;

use smartdevices::socket::SmartSocket;
use smartdevices::thermometer::SmartThermometer;
use smarthouse::room::SmartRoom;
use smarthouse::*;

use crate::smartdevices::SmartDevice;

fn main() {
    let mut socket1 = SmartDevice::from(SmartSocket::new("SmartSocket1", "brief of SmartSocket1"));
    let mut socket2 = SmartDevice::from(SmartSocket::new("SmartSocket2", "brief of SmartSocket2"));
    let mut socket3 = SmartDevice::from(SmartSocket::new("SmartSocket3", "brief of SmartSocket3"));
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

    let ret = house.add_room(room1);
    if ret.is_err() {
        panic!("{}", ret.unwrap_err())
    }

    let ret = house.add_room(room2);
    if ret.is_err() {
        panic!("{}", ret.unwrap_err())
    }

    let info_provider_1 = OwningDeviceInfoProvider {
        devices: vec![socket1],
    };

    let report1 = house.create_report(&info_provider_1);

    let info_provider_2 = BorrowingDeviceInfoProvider {
        devices: vec![&mut socket2, &mut thermo1, &mut socket3],
    };

    let report2 = house.create_report(&info_provider_2);

    println!("Report #1: {report1}");
    println!("Report #2: {report2}");
}
