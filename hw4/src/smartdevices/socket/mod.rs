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

    /// Returs brief as &str
    ///  # Example
    ///
    /// ```
    /// use homework4::smartdevices::socket::SmartSocket;
    ///
    /// let ss = SmartSocket::new("socket1", "brief");
    /// println!("{}", ss.brief())
    /// ```
    pub fn brief(&self) -> &str {
        &self.brief
    }

    /// Enables power
    ///  # Example
    ///
    /// ```
    /// use homework4::smartdevices::socket::SmartSocket;
    /// use homework4::smartdevices::SmartDevice;
    ///
    /// let mut ss = SmartSocket::new("socket1", "brief");
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
    /// use homework4::smartdevices::socket::SmartSocket;
    /// use homework4::smartdevices::SmartDevice;
    ///
    /// let mut ss = SmartSocket::new("socket1", "brief");
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
    /// use homework4::smartdevices::socket::SmartSocket;
    ///
    /// let ss = SmartSocket::new("socket1", "brief");
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
