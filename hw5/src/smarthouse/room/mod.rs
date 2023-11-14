use core::fmt;
use std::error::Error;
mod tests;

use crate::smartdevices::{
    socket::SmartSocketError, thermometer::SmartThermometerError, SmartDevice, SmartDeviceError,
};
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
    /// use hw5::smarthouse::room::SmartRoom;
    /// use hw5::smartdevices::SmartDevice;
    /// use hw5::smartdevices::socket::SmartSocket;
    ///
    /// let mut ss = SmartSocket::new("socket1", "brief");
    /// let mut sr = SmartRoom::new ("room1");
    /// assert!(sr.add_device(SmartDevice::from(&mut ss)).is_ok())
    /// ```
    pub fn add_device(&mut self, dev: SmartDevice) -> Result<&'static str, SmartRoomError> {
        let dev_name = match dev {
            SmartDevice::Socket(s) => {
                s.set_room(&self.name)?;
                s.get_name()
            }
            SmartDevice::Thermometer(t) => {
                t.set_room(&self.name)?;
                t.get_name()
            }
        };

        if self.devices.contains(&dev_name) {
            return Err(SmartRoomError {
                reason: format!("The room already contains device with name:{}", dev_name),
            });
        }
        self.devices.insert(dev_name);
        Ok("Success")
    }
}

#[derive(Debug)]
pub struct SmartRoomError {
    pub reason: String,
}

impl Error for SmartRoomError {}

impl fmt::Display for SmartRoomError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.reason)
    }
}

impl From<SmartSocketError> for SmartRoomError {
    fn from(value: SmartSocketError) -> Self {
        Self {
            reason: format!("SmartSocketError:{}", value.reason),
        }
    }
}

impl From<SmartThermometerError> for SmartRoomError {
    fn from(value: SmartThermometerError) -> Self {
        Self {
            reason: format!("SmartThermometerError:{}", value.reason),
        }
    }
}

impl From<SmartDeviceError> for SmartRoomError {
    fn from(value: SmartDeviceError) -> Self {
        match value {
            SmartDeviceError::Socket(s) => Self {
                reason: format!("SmartSocketError:{}", s.reason),
            },
            SmartDeviceError::Thermometer(t) => Self {
                reason: format!("SmartThermometerError:{}", t.reason),
            },
        }
    }
}
