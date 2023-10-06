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
