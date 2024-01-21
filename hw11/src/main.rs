#![allow(dead_code)]

mod integration_tests;

mod smartdevices;
mod smarthouse;

use smartdevices::socket::SmartSocket;
use smartdevices::thermometer::SmartThermometer;
use smarthouse::room::SmartRoom;
use smarthouse::*;

fn main() {
    let mut socket1 = SmartSocket::new("SmartSocket1", "brief of SmartSocket1").unwrap();
    let mut socket2 = SmartSocket::new("SmartSocket2", "brief of SmartSocket2").unwrap();
    let socket3 = SmartSocket::new("SmartSocket3", "brief of SmartSocket3").unwrap();

    let mut thermo = SmartThermometer::new("SmartThermometer1", -60.0, 60.0).unwrap();

    let mut house = SmartHouse::new("Sweet Home").unwrap();
    let mut room1 = SmartRoom::new("Room1").unwrap();
    let mut room2 = SmartRoom::new("Room2").unwrap();

    room1.add_device(&mut socket1).unwrap();
    room2.add_device(&mut socket2).unwrap();
    room2.add_device(&mut thermo).unwrap();

    house.add_room(room1).unwrap();
    house.add_room(room2).unwrap();

    let mut info_provider_1 = OwningDeviceInfoProvider {
        devices: Vec::new(),
    };

    info_provider_1.devices.push(Box::new(socket1));
    let report1 = house.create_report(&info_provider_1);

    let mut info_provider_2 = BorrowingDeviceInfoProvider {
        devices: Vec::new(),
    };

    info_provider_2.devices.push(&socket2);
    info_provider_2.devices.push(&thermo);
    info_provider_2.devices.push(&socket3);
    let report2 = house.create_report(&info_provider_2);

    println!("Report #1: {report1}");
    println!("Report #2: {report2}");
}
