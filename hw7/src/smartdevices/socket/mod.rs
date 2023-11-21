use core::fmt;
use std::error::Error;

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
    /// use hw7::smartdevices::socket::SmartSocket;
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
    /// use hw7::smartdevices::socket::SmartSocket;
    /// use hw7::smartdevices::SmartDevice;
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
    /// use hw7::smartdevices::socket::SmartSocket;
    /// use hw7::smartdevices::SmartDevice;
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
    /// use hw7::smartdevices::socket::SmartSocket;
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

    pub fn get_name(&self) -> String {
        self.name.clone()
    }
    pub fn get_state(&self) -> String {
        if self.state {
            format!("Power consumption:{:.2}", self.power_consumption())
        } else {
            "Socket off".to_string()
        }
    }
    pub fn set_room(&mut self, name: &str) -> Result<String, SmartSocketError> {
        if self.room.is_some() {
            Err(SmartSocketError {
                reason: format!("Device installed into room:{}", self.get_room()),
            })
        } else {
            self.room = Some(String::from(name));
            Ok(format!(
                "Device has installed into room:{}",
                self.get_room()
            ))
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
pub struct SmartSocketError {
    pub reason: String,
}

impl Error for SmartSocketError {}

impl fmt::Display for SmartSocketError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.reason)
    }
}
