pub mod room;

use crate::smartdevices::SmartDevice;
use room::SmartRoom;
use std::collections::HashMap;

use self::room::SmartRoomError;

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
    /// Add SmartRoom into house
    ///  # Example
    ///
    /// ```
    /// use hw6::smarthouse::room::SmartRoom;
    /// use hw6::smarthouse::SmartHouse;
    ///
    /// let mut sh = SmartHouse::new ("house1");
    /// let mut sr = SmartRoom::new ("room1");
    /// sh.add_room(sr);
    ///
    /// assert_eq!(sh.count_rooms(), 1);
    /// ```
    pub fn add_room(&mut self, room: SmartRoom) -> Result<&str, SmartRoomError> {
        let found = self.rooms.contains_key(&room.name);
        if found {
            return Err(SmartRoomError {
                reason: String::from("Can't add room. Room is present"),
            });
        }
        self.rooms.insert(room.name.clone(), room);
        Ok("Success")
    }

    /// Del SmartRoom from house
    /// # Examples
    ///
    /// ```
    /// use hw6::smarthouse::room::SmartRoom;
    /// use hw6::smarthouse::SmartHouse;
    ///
    /// let mut sh = SmartHouse::new ("house1");
    /// let mut sr = SmartRoom::new ("room1");
    /// sh.add_room(sr);
    ///
    /// assert_eq!(sh.count_rooms(), 1);
    /// sh.del_room(String::from("room1"));
    /// assert_eq!(sh.count_rooms(), 0);
    /// ```
    pub fn del_room(&mut self, name_room: String) -> Result<&str, SmartRoomError> {
        let room = self.rooms.remove(&name_room);
        if room.is_none() {
            return Err(SmartRoomError {
                reason: format!("Room {name_room} not found"),
            });
        }
        Ok("Success")
    }

    /// Returns Vec with names of rooms
    ///  # Example
    ///
    /// ```
    /// use hw6::smarthouse::room::SmartRoom;
    /// use hw6::smarthouse::SmartHouse;
    ///
    /// let mut sh = SmartHouse::new ("house1");
    /// let mut sr = SmartRoom::new ("room1");
    /// sh.add_room(sr);
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
    /// use hw6::smartdevices::socket::SmartSocket;
    /// use hw6::smartdevices::SmartDevice;
    /// use hw6::smarthouse::room::SmartRoom;
    /// use hw6::smarthouse::SmartHouse;
    ///
    /// let mut sh = SmartHouse::new ("house1");
    /// let mut sr = SmartRoom::new ("room1");
    /// let mut ss = SmartSocket::new("socket1", "brief");
    /// sr.add_device(SmartDevice::from(&mut ss));
    /// sh.add_room(sr);
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
        let mut report = self.name.clone();
        report.push('\n');
        let rooms = self.get_rooms();
        for room in rooms {
            let devices = self.devices(&room);
            for name in devices {
                let sub_report = provider.get_device_state(room.clone(), name);
                report.push_str(&sub_report);
                report.push('\n');
            }
        }
        report
    }

    pub fn count_rooms(&self) -> usize {
        self.rooms.len()
    }
}

pub trait DeviceInfoProvider {
    fn get_device_state(&self, room: String, name: String) -> String;
}

pub struct OwningDeviceInfoProvider<'a> {
    pub devices: Vec<SmartDevice<'a>>,
}

pub struct BorrowingDeviceInfoProvider<'a> {
    pub devices: Vec<&'a SmartDevice<'a>>,
}

//реализация трейта `DeviceInfoProvider` для поставщиков информации
impl DeviceInfoProvider for OwningDeviceInfoProvider<'_> {
    fn get_device_state(&self, room: String, name: String) -> String {
        let mut found = false;
        let mut report: String = format!("Report for room:{room}\n");
        for d in self.devices.iter() {
            let (r, n, s) = match d {
                SmartDevice::Socket(s) => (s.get_room(), s.get_name(), s.get_state()),
                SmartDevice::Thermometer(t) => (t.get_room(), t.get_name(), t.get_state()),
            };
            if r == room && n == name {
                found = true;
                report.push_str(format!("   => Device[{}]:{}\n", n, s).as_str());
            }
        }
        if !found {
            let s = "SmartDevice not found";
            report.push_str(format!("   => Device[{}]:{}\n", name, s).as_str());
        }
        report
    }
}

impl DeviceInfoProvider for BorrowingDeviceInfoProvider<'_> {
    fn get_device_state(&self, room: String, name: String) -> String {
        let mut found = false;
        let mut report: String = format!("Report for room:{room}\n");
        for d in self.devices.iter() {
            let (r, n, s) = match d {
                SmartDevice::Socket(s) => (s.get_room(), s.get_name(), s.get_state()),
                SmartDevice::Thermometer(t) => (t.get_room(), t.get_name(), t.get_state()),
            };
            if r == room && n == name {
                found = true;
                report.push_str(format!("   => Device[{}]:{}\n", n, s).as_str());
            }
        }
        if !found {
            let s = "SmartDevice not found";
            report.push_str(format!("   => Device[{}]:{}\n", name, s).as_str());
        }
        report
    }
}
