//! cargo run --example example1

use ::hw6::smartdevices::socket::SmartSocket;
use ::hw6::smartdevices::thermometer::SmartThermometer;
use ::hw6::smartdevices::SmartDevice;
use ::hw6::smarthouse::room::SmartRoom;
use ::hw6::smarthouse::SmartHouse;
use ::hw6::smarthouse::*;

fn main() {
    let mut socket1 = SmartSocket::new("SmartSocket1", "brief of SmartSocket1");

    let mut socket2 = SmartSocket::new("SmartSocket2", "brief of SmartSocket2");
    let mut socket3 = SmartSocket::new("SmartSocket3", "brief of SmartSocket3");

    let mut thermo = SmartThermometer::new("SmartThermometer1", -60.0, 60.0);

    let mut house = SmartHouse::new("Sweet Home");
    let mut room1 = SmartRoom::new("Room1");
    let mut room2 = SmartRoom::new("Room2");

    let _ = room1.add_device(SmartDevice::from(&mut socket1));
    let _ = room2.add_device(SmartDevice::from(&mut socket2));
    let _ = room2.add_device(SmartDevice::from(&mut thermo));

    let ret = house.add_room(room1);
    if ret.is_err() {
        panic!("{}", ret.unwrap_err())
    }
    let ret = house.add_room(room2);
    if ret.is_err() {
        panic!("{}", ret.unwrap_err())
    }

    let mut info_provider_1 = OwningDeviceInfoProvider {
        devices: Vec::new(),
    };

    info_provider_1
        .devices
        .push(SmartDevice::from(&mut socket1));
    let report1 = house.create_report(&info_provider_1);

    let mut info_provider_2 = BorrowingDeviceInfoProvider {
        devices: Vec::new(),
    };

    let sd_socket2 = SmartDevice::from(&mut socket2);
    let sd_socket3 = SmartDevice::from(&mut socket3);
    let sd_thermo = SmartDevice::from(&mut thermo);

    info_provider_2.devices.push(&sd_socket2);
    info_provider_2.devices.push(&sd_thermo);
    info_provider_2.devices.push(&sd_socket3);
    let report2 = house.create_report(&info_provider_2);

    println!("Report #1: {report1}");
    println!("Report #2: {report2}");
}
