use crate::SmartDevice;

#[derive(Default)]
pub struct SmartSocket {
    brief: String,
    state: bool,
    power: f32,
    name: String,
    room: Option<String>,
}

impl SmartSocket {
    pub fn new(name: String, brief: String) -> Self {
        Self {
            name,
            brief,
            state: false,
            power: 0.0,
            room: None,
        }
    }
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
        self.power
    }
}

impl SmartDevice for SmartSocket {
    fn get_name(&self) -> &str {
        self.name.as_str()
    }
    fn get_state(&self) -> String {
        format!("Power consumption:{}", self.power_consumption())
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
