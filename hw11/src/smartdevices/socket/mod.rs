use crate::smartdevices::SmartDevice;
use rand::Rng;
use thiserror::Error;

use super::SmartDeviceError;

mod tests;

#[derive(Debug, Error, PartialEq)]
pub enum SmartSocketError {
    #[error("missing name of socket")]
    MissName,
    #[error("missing brief of socket")]
    MissBrief,
    #[error("can't set room, socket is installed in room:{0}")]
    SetRoom(String),
}

#[derive(Default, Debug)]
pub struct SmartSocket {
    brief: String,
    state: bool,
    _power: f32,
    name: String,
    room: Option<String>,
}

impl SmartSocket {
    pub fn new(name: &str, brief: &str) -> Result<Self, SmartSocketError> {
        if name.is_empty() {
            return Err(SmartSocketError::MissName);
        }

        if brief.is_empty() {
            return Err(SmartSocketError::MissBrief);
        }

        Ok(Self {
            name: String::from(name),
            brief: String::from(brief),
            state: false,
            _power: 0.0,
            room: None,
        })
    }

    /// Returs brief as &str
    ///  # Example
    ///
    /// ```
    /// use hw11::smartdevices::socket::SmartSocket;
    ///
    /// let ss = SmartSocket::new("socket1", "brief").unwrap();
    /// println!("{}", ss.brief())
    /// ```
    pub fn brief(&self) -> &str {
        &self.brief
    }

    /// Enables power
    ///  # Example
    ///
    /// ```
    /// use hw11::smartdevices::socket::SmartSocket;
    /// use hw11::smartdevices::SmartDevice;
    ///
    /// let mut ss = SmartSocket::new("socket1", "brief").unwrap();
    /// ss.on();
    /// println!("{}", ss.get_state())
    /// ```
    pub fn on(&mut self) {
        self.state = true;
    }

    /// Disables power
    ///  # Example
    ///
    /// ```
    /// use hw11::smartdevices::socket::SmartSocket;
    /// use hw11::smartdevices::SmartDevice;
    ///
    /// let mut ss = SmartSocket::new("socket1", "brief").unwrap();
    /// ss.off();
    /// println!("{}", ss.get_state())
    /// ```
    pub fn off(&mut self) {
        self.state = false;
    }

    /// Returns power consumption as f32 value
    ///  # Example
    ///
    /// ```
    /// use hw11::smartdevices::socket::SmartSocket;
    ///
    /// let ss = SmartSocket::new("socket1", "brief").unwrap();
    /// println!("{}", ss.power_consumption());
    /// ```
    pub fn power_consumption(&self) -> f32 {
        if self.state {
            rand::thread_rng().gen_range(0.01..220.00)
        } else {
            0.0
        }
    }
}

impl From<SmartSocketError> for SmartDeviceError {
    fn from(value: SmartSocketError) -> Self {
        Self::Error(format!("{}", value))
    }
}

impl SmartDevice for SmartSocket {
    fn get_name(&self) -> &str {
        self.name.as_str()
    }
    fn get_state(&self) -> String {
        if self.state {
            format!("Power consumption:{:.2}", self.power_consumption())
        } else {
            "Socket off".to_string()
        }
    }
    fn set_room(&mut self, name: &str) -> Result<(), SmartDeviceError> {
        if self.room.is_some() {
            let room = self.get_room().unwrap();
            Err(SmartSocketError::SetRoom(room).into())
        } else {
            self.room = Some(String::from(name));
            Ok(())
        }
    }
    fn get_room(&self) -> Result<String, SmartDeviceError> {
        if self.room.is_none() {
            return Err(SmartDeviceError::NotInstalled);
        }
        Ok(self.room.as_ref().unwrap().clone())
    }
}
