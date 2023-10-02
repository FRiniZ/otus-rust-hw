#![allow(dead_code)]
pub mod sdevice;
pub mod ssocket;
pub mod sthermometer;

use std::collections::HashMap;

use sdevice::SmartDevice;
use ssocket::SmartSocket;
use sthermometer::SmartThermometer;

struct SmartRoom {
    name: String,
    devices: HashMap<String, String>,
}

struct SmartHouse {
    name: String,
    rooms: HashMap<String, SmartRoom>,
}

impl SmartRoom {
    fn new(name: String) -> Self {
        Self {
            name,
            devices: HashMap::new(),
        }
    }
    fn add_device(&mut self, dev: &mut impl SmartDevice) -> bool {
        let ret = dev.set_room(self.name.as_str());
        if ret {
            let name = dev.get_name().to_string();
            let key = name.clone();
            self.devices.insert(key, name);
            true
        } else {
            false
        }
    }
}

impl SmartHouse {
    fn new(name: String) -> Self {
        Self {
            name,
            rooms: HashMap::new(),
        }
    }

    fn add_room(&mut self, room: SmartRoom) {
        self.rooms.insert(room.name.clone(), room);
    }

    fn get_rooms(&self) -> Vec<String> {
        let mut rc: Vec<String> = Vec::new();
        for room in self.rooms.iter() {
            rc.push(room.0.clone());
        }
        rc
    }

    fn devices(&self, room: &str) -> Vec<String> {
        let mut rc: Vec<String> = Vec::new();

        for r in self.rooms.iter() {
            if r.0 == room {
                for d in r.1.devices.iter() {
                    rc.push(d.1.to_string());
                }
            }
        }
        rc
    }

    fn create_report(&self, provider: &impl DeviceInfoProvider) -> String {
        /*перебор комнат и устройств в них для составления отчёта */
        provider.get_device_state(self)
    }
}
trait DeviceInfoProvider {
    fn get_device_state(&self, house: &SmartHouse) -> String;
}

struct OwningDeviceInfoProvider {
    devices: Vec<Box<dyn SmartDevice>>,
}

struct BorrowingDeviceInfoProvider<'a> {
    devices: Vec<&'a dyn SmartDevice>,
}

//реализация трейта `DeviceInfoProvider` для поставщиков информации
impl DeviceInfoProvider for OwningDeviceInfoProvider {
    fn get_device_state(&self, house: &SmartHouse) -> String {
        let mut report = house.name.clone();
        report.push('\n');
        for d in self.devices.iter() {
            report.push_str(
                format!(
                    "Room[{}] => device[{}]:{}\n",
                    d.get_room(),
                    d.get_name(),
                    d.get_state()
                )
                .as_str(),
            );
        }
        report
    }
}

impl DeviceInfoProvider for BorrowingDeviceInfoProvider<'_> {
    fn get_device_state(&self, house: &SmartHouse) -> String {
        let mut report = house.name.clone();
        report.push('\n');
        for d in self.devices.iter() {
            report.push_str(
                format!(
                    "Room[{}] => device[{}]:{}\n",
                    d.get_room(),
                    d.get_name(),
                    d.get_state()
                )
                .as_str(),
            );
        }
        report
    }
}

fn main() {
    let mut socket1 = SmartSocket::new(
        String::from("SmartSocket1"),
        String::from("brief of SmartSocket1"),
    );

    let mut socket2 = SmartSocket::new(
        String::from("SmartSocket2"),
        String::from("brief of SmartSocket2"),
    );

    let mut thermo = SmartThermometer::new(String::from("SmartThermometer1"), -60.0, 60.0);

    let mut house = SmartHouse::new(String::from("Sweet Home"));
    let mut room1 = SmartRoom::new(String::from("Room1"));
    let mut room2 = SmartRoom::new(String::from("Room2"));

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
    let report2 = house.create_report(&info_provider_2);

    println!("Report #1: {report1}");
    println!("Report #2: {report2}");
}
