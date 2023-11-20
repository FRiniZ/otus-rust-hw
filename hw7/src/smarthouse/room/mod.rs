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
    /// use hw7::smarthouse::room::SmartRoom;
    /// use hw7::smartdevices::SmartDevice;
    /// use hw7::smartdevices::socket::SmartSocket;
    ///
    /// let mut ss = SmartSocket::new("socket1", "brief");
    /// let mut sr = SmartRoom::new ("room1");
    /// assert!(sr.add_device(SmartDevice::from(&mut ss)).is_ok())
    /// ```
    pub fn add_device(&mut self, dev: &mut SmartDevice) -> Result<&'static str, SmartRoomError> {
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

    /// Del SmartDevice from room
    ///  # Example
    ///
    /// ```
    /// use hw7::smarthouse::room::SmartRoom;
    /// use hw7::smartdevices::SmartDevice;
    /// use hw7::smartdevices::socket::SmartSocket;
    ///
    /// let mut ss = SmartSocket::new("socket1", "brief");
    /// let mut sr = SmartRoom::new ("room1");
    /// assert!(sr.add_device(SmartDevice::from(&mut ss)).is_ok());
    /// assert!(sr.del_device(String::from("socket1")).is_ok());
    /// assert!(sr.del_device(String::from("socket2")).is_err());
    /// ```
    pub fn del_device(&mut self, dev_name: String) -> Result<&'static str, SmartRoomError> {
        if !self.devices.remove(&dev_name) {
            return Err(SmartRoomError {
                reason: format!("The room doesn't contain device with name:{}", dev_name),
            });
        }
        Ok("Success")
    }

    /// List of SmartDevices in Room
    ///  # Example
    ///
    /// ```
    /// use hw7::smarthouse::room::SmartRoom;
    /// use hw7::smartdevices::SmartDevice;
    /// use hw7::smartdevices::socket::SmartSocket;
    /// use hw7::smartdevices::thermometer::SmartThermometer;
    ///
    /// let mut ss = SmartSocket::new("socket1", "brief");
    /// let mut st = SmartThermometer::new("thermo1", -60.0, 60.0);
    /// let mut sr = SmartRoom::new ("room1");
    /// assert!(sr.add_device(SmartDevice::from(&mut ss)).is_ok());
    /// assert!(sr.add_device(SmartDevice::from(&mut st)).is_ok());
    /// assert_eq!(sr.devices().len() , 2);
    /// ```
    pub fn devices(&mut self) -> Vec<String> {
        let mut rc: Vec<String> = Vec::new();

        for dev_name in self.devices.iter() {
            rc.push(dev_name.to_string())
        }
        rc
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
