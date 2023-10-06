pub mod socket;
pub mod thermometer;

/// Common interfaces for all smart devices
pub trait SmartDevice {
    /// Returns name of device
    fn get_name(&self) -> &str;

    /// Returns state of device
    fn get_state(&self) -> String;

    /// Installed device into room
    fn set_room(&mut self, name: &str) -> bool;

    /// Returns name of room where installed device
    fn get_room(&self) -> String;
}
