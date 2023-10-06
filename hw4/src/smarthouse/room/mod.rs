mod tests;

use crate::smartdevices::SmartDevice;
use std::collections::HashSet;

pub struct SmartRoom {
    pub name: String,
    pub devices: HashSet<String>,
}

impl SmartRoom {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            devices: HashSet::new(),
        }
    }

    /// Add SmartDevice into room
    ///  # Example
    ///
    /// ```
    /// use homework4::smarthouse::room::SmartRoom;
    /// use homework4::smartdevices::socket::SmartSocket;
    ///
    /// let mut ss = SmartSocket::new("socket1", "brief");
    /// let mut sr = SmartRoom::new ("room1");
    /// assert!(sr.add_device(&mut ss))
    /// ```
    pub fn add_device(&mut self, dev: &mut impl SmartDevice) -> bool {
        if self.devices.contains(dev.get_name()) {
            return false;
        }
        let ret = dev.set_room(self.name.as_str());
        if ret {
            let name = dev.get_name().to_string();
            self.devices.insert(name);
            true
        } else {
            false
        }
    }
}
