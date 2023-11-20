use self::{socket::SmartSocket, socket::SmartSocketError};
use self::{thermometer::SmartThermometer, thermometer::SmartThermometerError};

pub mod socket;
pub mod thermometer;

pub enum SmartDevice {
    Socket(SmartSocket),
    Thermometer(SmartThermometer),
}

pub enum SmartDeviceError {
    Socket(SmartSocketError),
    Thermometer(SmartThermometerError),
}

impl From<SmartSocketError> for SmartDeviceError {
    fn from(value: SmartSocketError) -> Self {
        Self::Socket(value)
    }
}

impl From<SmartThermometerError> for SmartDeviceError {
    fn from(value: SmartThermometerError) -> Self {
        Self::Thermometer(value)
    }
}

impl From<SmartSocket> for SmartDevice {
    fn from(value: SmartSocket) -> Self {
        Self::Socket(value)
    }
}

impl From<SmartThermometer> for SmartDevice {
    fn from(value: SmartThermometer) -> Self {
        Self::Thermometer(value)
    }
}
