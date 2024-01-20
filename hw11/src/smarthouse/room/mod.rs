mod tests;

use crate::smartdevices::{SmartDevice, SmartDeviceError};
use std::collections::HashSet;
use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum SmartRoomError {
    #[error("missing name of room")]
    MissName,
    #[error("error:{0}")]
    Error(String),
    #[error("can't add device:{0}")]
    AddDeviceErr(#[from] SmartDeviceError),
}

pub struct SmartRoom {
    pub name: String,
    pub devices: HashSet<String>,
}

impl SmartRoom {
    pub fn new(name: &str) -> Result<Self, SmartRoomError> {
        if name.is_empty() {
            return Err(SmartRoomError::MissName);
        }
        Ok(Self {
            name: name.to_string(),
            devices: HashSet::new(),
        })
    }

    /// Add SmartDevice into room
    ///  # Example
    ///
    /// ```
    /// use hw11::smarthouse::room::SmartRoom;
    /// use hw11::smartdevices::socket::SmartSocket;
    ///
    /// let mut ss = SmartSocket::new("socket1", "brief").unwrap();
    /// let mut sr = SmartRoom::new ("room1").unwrap();
    /// assert!(sr.add_device(&mut ss).is_ok())
    /// ```
    pub fn add_device(&mut self, dev: &mut impl SmartDevice) -> Result<(), SmartRoomError> {
        let dev_name = dev.get_name();
        if self.devices.contains(dev_name) {
            return Err(SmartRoomError::Error(format!(
                "room has device with the name:{}",
                dev_name
            )));
        }
        let ret = dev.set_room(self.name.as_str());

        match ret {
            Ok(_) => {
                let name = dev.get_name().to_string();
                self.devices.insert(name);
                Ok(())
            }
            Err(e) => Err(e.into()),
        }
    }
}
