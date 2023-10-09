//! cargo run --example example1

use ::homework4::smartdevices::socket::SmartSocket;
use ::homework4::smartdevices::thermometer::SmartThermometer;
use ::homework4::smarthouse::room::SmartRoom;
use ::homework4::smarthouse::SmartHouse;
use ::homework4::smarthouse::*;

fn main() {
    let mut socket1 = SmartSocket::new("SmartSocket1", "brief of SmartSocket1");

    let mut socket2 = SmartSocket::new("SmartSocket2", "brief of SmartSocket2");
    let socket3 = SmartSocket::new("SmartSocket3", "brief of SmartSocket3");

    let mut thermo = SmartThermometer::new("SmartThermometer1", -60.0, 60.0);

    let mut house = SmartHouse::new("Sweet Home");
    let mut room1 = SmartRoom::new("Room1");
    let mut room2 = SmartRoom::new("Room2");

    room1.add_device(&mut socket1);
    room2.add_device(&mut socket2);
    room2.add_device(&mut thermo);

    house.add_room(room1);
    house.add_room(room2);

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
