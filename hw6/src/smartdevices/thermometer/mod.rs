use core::fmt;
use std::error::Error;

mod tests;

#[derive(Default, Debug)]
pub struct SmartThermometer {
    _level_min: f32,
    _level_max: f32,
    temperature: f32,
    name: String,
    room: Option<String>,
}

impl SmartThermometer {
    pub fn new(name: &str, min: f32, max: f32) -> Self {
        Self {
            name: name.to_string(),
            _level_min: min,
            _level_max: max,
            temperature: 0.0,
            room: None,
        }
    }

    /// Returs current temperature as f32
    ///  # Example
    ///
    /// ```
    /// use hw6::smartdevices::thermometer::SmartThermometer;
    ///
    /// let ss = SmartThermometer::new("thermometer1", -40.0, 60.0);
    /// println!("{}", ss.temperature())
    /// ```
    pub fn temperature(&self) -> &f32 {
        &self.temperature
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }
    pub fn get_state(&self) -> String {
        format!("Temperature:{}", self.temperature)
    }
    pub fn set_room(&mut self, name: &str) -> Result<String, SmartThermometerError> {
        if self.room.is_some() {
            Err(SmartThermometerError {
                reason: format!("Device installed into room:{}", self.get_room()),
            })
        } else {
            self.room = Some(String::from(name));
            Ok(format!("Device installed into room:{}", self.get_room()))
        }
    }
    pub fn get_room(&self) -> String {
        if self.room.is_some() {
            self.room.as_ref().unwrap().clone()
        } else {
            String::from("Not installed")
        }
    }
}

#[derive(Debug)]
pub struct SmartThermometerError {
    pub reason: String,
}

impl Error for SmartThermometerError {}

impl fmt::Display for SmartThermometerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.reason)
    }
}
