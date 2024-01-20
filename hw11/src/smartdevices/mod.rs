pub mod socket;
pub mod thermometer;

use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum SmartDeviceError {
    #[error("error in device:{0}")]
    Error(String),
    #[error("device not installed in room")]
    NotInstalled,
    #[error("device installed in room:{0}")]
    Installed(String),
}

/// Common interfaces for all smart devices
pub trait SmartDevice {
    /// Returns name of device
    fn get_name(&self) -> &str;

    /// Returns state of device
    fn get_state(&self) -> String;

    /// Installed device into room
    fn set_room(&mut self, name: &str) -> Result<(), SmartDeviceError>;

    /// Returns name of room where installed device
    fn get_room(&self) -> Result<String, SmartDeviceError>;
}
