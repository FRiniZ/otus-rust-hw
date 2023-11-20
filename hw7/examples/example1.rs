//! cargo run --example example1

use ::hw7::smartdevices::socket::SmartSocket;
use ::hw7::smartdevices::thermometer::SmartThermometer;
use ::hw7::smartdevices::SmartDevice;
use ::hw7::smarthouse::room::SmartRoom;
use ::hw7::smarthouse::SmartHouse;
use ::hw7::smarthouse::*;

fn main() {
    let mut socket1 = SmartDevice::from(SmartSocket::new("SmartSocket1", "brief of SmartSocket1"));
    let mut socket2 = SmartDevice::from(SmartSocket::new("SmartSocket2", "brief of SmartSocket2"));
    let mut socket3 = SmartDevice::from(SmartSocket::new("SmartSocket3", "brief of SmartSocket3"));
    let mut thermo1 = SmartDevice::from(SmartThermometer::new("SmartThermometer1", -60.0, 60.0));

    let mut house = SmartHouse::new("Sweet Home");
    let mut room1 = SmartRoom::new("Room1");
    let mut room2 = SmartRoom::new("Room2");

    let _ = room1.add_device(&mut socket1);
    let _ = room2.add_device(&mut socket2);
    let _ = room2.add_device(&mut thermo1);

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

    info_provider_1.devices.push(socket1);
    let report1 = house.create_report(&info_provider_1);

    let mut info_provider_2 = BorrowingDeviceInfoProvider {
        devices: Vec::new(),
    };

    info_provider_2.devices.push(&mut socket2);
    info_provider_2.devices.push(&mut thermo1);
    info_provider_2.devices.push(&mut socket3);
    let report2 = house.create_report(&info_provider_2);

    println!("Report #1: {report1}");
    println!("Report #2: {report2}");
}
