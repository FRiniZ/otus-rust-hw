#[derive(Default)]
pub struct SmartThermometer {
    _level_min: f32,
    _level_max: f32,
    temperature: f32,
}

impl SmartThermometer {
    pub fn new(min: f32, max: f32) -> Self {
        Self {
            _level_min: min,
            _level_max: max,
            temperature: 0.0,
        }
    }

    pub fn temperature(&self) -> &f32 {
        &self.temperature
    }
}
