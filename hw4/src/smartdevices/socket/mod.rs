use crate::smartdevices::SmartDevice;
use rand::Rng;

mod tests;

#[derive(Default, Debug)]
pub struct SmartSocket {
    brief: String,
    state: bool,
    _power: f32,
    name: String,
    room: Option<String>,
}

impl SmartSocket {
    pub fn new(name: &str, brief: &str) -> Self {
        Self {
            name: String::from(name),
            brief: String::from(brief),
            state: false,
            _power: 0.0,
            room: None,
        }
    }

    /// Return brief as &str
    ///  # Examples
    ///
    /// ```
    /// let ss = SmartSocket::new("socket1", "brief");
    /// println!("{}", ss.brief)
    /// ```
    pub fn brief(&self) -> &str {
        &self.brief
    }
    pub fn on(&mut self) {
        self.state = true;
    }
    pub fn off(&mut self) {
        self.state = false;
    }
    pub fn power_consumption(&self) -> f32 {
        if self.state {
            rand::thread_rng().gen_range(0.01..220.00)
        } else {
            0.0
        }
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
