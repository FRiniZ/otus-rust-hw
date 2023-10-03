pub trait SmartDevice {
    fn get_name(&self) -> &str;
    fn get_state(&self) -> String;
    fn set_room(&mut self, name: &str) -> bool;
    fn get_room(&self) -> String;
}
