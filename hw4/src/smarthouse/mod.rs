pub mod room;

use crate::smartdevices::SmartDevice;
use room::SmartRoom;
use std::collections::HashMap;

pub struct SmartHouse {
    name: String,
    rooms: HashMap<String, SmartRoom>,
}

impl SmartHouse {
    pub fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
            rooms: HashMap::new(),
        }
    }

    pub fn add_room(&mut self, room: SmartRoom) {
        self.rooms.insert(room.name.clone(), room);
    }

    pub fn get_rooms(&self) -> Vec<String> {
        let mut rc: Vec<String> = Vec::new();
        for room in self.rooms.iter() {
            rc.push(room.0.clone());
        }
        rc
    }

    pub fn devices(&self, room: &str) -> Vec<String> {
        let mut rc: Vec<String> = Vec::new();

        for (key, val) in self.rooms.iter() {
            if key == room {
                for d in val.devices.iter() {
                    rc.push(d.to_string());
                }
            }
        }
        rc
    }

    pub fn create_report(&self, provider: &impl DeviceInfoProvider) -> String {
        provider.get_device_state(self)
    }
}

pub trait DeviceInfoProvider {
    fn get_device_state(&self, house: &SmartHouse) -> String;
}

pub struct OwningDeviceInfoProvider {
    pub devices: Vec<Box<dyn SmartDevice>>,
}

pub struct BorrowingDeviceInfoProvider<'a> {
    pub devices: Vec<&'a dyn SmartDevice>,
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
