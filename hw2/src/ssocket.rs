#[derive(Default)]
pub struct SmartSocket {
    brief: String,
    state: bool,
    _power: f32,
}

impl SmartSocket {
    pub fn new(brief: String) -> Self {
        Self {
            brief,
            state: false,
            _power: 0.0,
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
    pub fn power_consumption(self) -> f32 {
        todo!()
    }
}
