use super::SmartDeviceError;
use crate::smartdevices::SmartDevice;
use thiserror::Error;

mod tests;

#[derive(Debug, Error, PartialEq)]
pub enum SmartThermometerError {
    #[error("missing name of thermometer")]
    MissName,
    #[error("can't set room, thermometer is installed in room:{0}")]
    SetRoom(String),
}

#[derive(Default)]
pub struct SmartThermometer {
    _level_min: f32,
    _level_max: f32,
    temperature: f32,
    name: String,
    room: Option<String>,
}

impl SmartThermometer {
    pub fn new(name: &str, min: f32, max: f32) -> Result<Self, SmartThermometerError> {
        if name.is_empty() {
            return Err(SmartThermometerError::MissName);
        }

        Ok(Self {
            name: name.to_string(),
            _level_min: min,
            _level_max: max,
            temperature: 0.0,
            room: None,
        })
    }

    /// Returs current temperature as f32
    ///  # Example
    ///
    /// ```
    /// use hw11::smartdevices::thermometer::SmartThermometer;
    ///
    /// let ss = SmartThermometer::new("thermometer1", -40.0, 60.0).unwrap();
    /// println!("{}", ss.temperature())
    /// ```
    pub fn temperature(&self) -> &f32 {
        &self.temperature
    }
}

impl From<SmartThermometerError> for SmartDeviceError {
    fn from(value: SmartThermometerError) -> Self {
        Self::Error(format!("{}", value))
    }
}

impl SmartDevice for SmartThermometer {
    fn get_name(&self) -> &str {
        self.name.as_str()
    }
    fn get_state(&self) -> String {
        format!("Temperature:{}", self.temperature)
    }
    fn set_room(&mut self, name: &str) -> Result<(), SmartDeviceError> {
        if self.room.is_some() {
            let room = self.get_room().unwrap();
            Err(SmartThermometerError::SetRoom(room).into())
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
