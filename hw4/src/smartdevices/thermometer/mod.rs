use crate::smartdevices::SmartDevice;

mod tests;

#[derive(Default)]
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
    /// use homework4::smartdevices::thermometer::SmartThermometer;
    ///
    /// let ss = SmartThermometer::new("thermometer1", -40.0, 60.0);
    /// println!("{}", ss.temperature())
    /// ```
    pub fn temperature(&self) -> &f32 {
        &self.temperature
    }
}

impl SmartDevice for SmartThermometer {
    fn get_name(&self) -> &str {
        self.name.as_str()
    }
    fn get_state(&self) -> String {
        format!("Temperature:{}", self.temperature)
    }
    fn set_room(&mut self, name: &str) -> bool {
        if self.room.is_some() {
            false
        } else {
            self.room = Some(String::from(name));
            true
        }
    }
    fn get_room(&self) -> String {
        if self.room.is_some() {
            self.room.as_ref().unwrap().clone()
        } else {
            String::from("Not installed")
        }
    }
}
