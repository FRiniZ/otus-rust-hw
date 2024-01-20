pub mod room;

use crate::smartdevices::SmartDevice;
use room::SmartRoom;
use std::collections::HashMap;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SmartHomeError {
    #[error("missing name of house")]
    MissName,
    #[error("name{0} of room is present")]
    AddRoomErr(String),
}

pub struct SmartHouse {
    name: String,
    rooms: HashMap<String, SmartRoom>,
}

impl SmartHouse {
    pub fn new(name: &str) -> Result<Self, SmartHomeError> {
        if name.is_empty() {
            return Err(SmartHomeError::MissName);
        }
        Ok(Self {
            name: String::from(name),
            rooms: HashMap::new(),
        })
    }

    /// Add SmartRoom into house
    ///  # Example
    ///
    /// ```
    /// use hw11::smarthouse::room::SmartRoom;
    /// use hw11::smarthouse::SmartHouse;
    ///
    /// let mut sh = SmartHouse::new ("house1").unwrap();
    /// let mut sr = SmartRoom::new ("room1").unwrap();
    /// sh.add_room(sr);
    ///
    /// assert_eq!(sh.count_rooms(), 1);
    /// ```
    pub fn add_room(&mut self, room: SmartRoom) -> Result<(), SmartHomeError> {
        let room_name = room.name.clone();

        if self.rooms.contains_key(room_name.as_str()) {
            return Err(SmartHomeError::AddRoomErr(room_name));
        }
        self.rooms.insert(room_name, room);
        Ok(())
    }

    /// Returns Vec with names of rooms
    ///  # Example
    ///
    /// ```
    /// use hw11::smarthouse::room::SmartRoom;
    /// use hw11::smarthouse::SmartHouse;
    ///
    /// let mut sh = SmartHouse::new ("house1").unwrap();
    /// let mut sr = SmartRoom::new ("room1").unwrap();
    /// sh.add_room(sr).unwrap();
    ///
    /// assert_eq!(sh.get_rooms(), vec!["room1"]);
    /// ```
    pub fn get_rooms(&self) -> Vec<String> {
        let mut rc: Vec<String> = Vec::new();
        for room in self.rooms.iter() {
            rc.push(room.0.clone());
        }
        rc
    }

    /// Returns Vec with names of devices from room
    ///  # Example
    ///
    /// ```
    /// use hw11::smartdevices::socket::SmartSocket;
    /// use hw11::smarthouse::room::SmartRoom;
    /// use hw11::smarthouse::SmartHouse;
    ///
    /// let mut sh = SmartHouse::new ("house1").unwrap();
    /// let mut sr = SmartRoom::new ("room1").unwrap();
    /// let mut ss = SmartSocket::new("socket1", "brief").unwrap();
    /// sr.add_device(&mut ss).unwrap();
    /// sh.add_room(sr).unwrap();
    ///
    /// assert_eq!(sh.devices("room1"), vec!["socket1"]);
    /// ```
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

    pub fn count_rooms(&self) -> usize {
        self.rooms.len()
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
            let room = match d.get_room() {
                Ok(r) => r,
                Err(e) => format!("{}", e),
            };
            report.push_str(
                format!(
                    "Room[{}] => device[{}]:{}\n",
                    room,
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
            let room = match d.get_room() {
                Ok(r) => r,
                Err(e) => format!("{}", e),
            };
            report.push_str(
                format!(
                    "Room[{}] => device[{}]:{}\n",
                    room,
                    d.get_name(),
                    d.get_state()
                )
                .as_str(),
            );
        }
        report
    }
}
