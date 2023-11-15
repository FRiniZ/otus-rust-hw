use self::{socket::SmartSocket, socket::SmartSocketError};
use self::{thermometer::SmartThermometer, thermometer::SmartThermometerError};

pub mod socket;
pub mod thermometer;

pub enum SmartDevice<'a> {
    Socket(&'a mut SmartSocket),
    Thermometer(&'a mut SmartThermometer),
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

impl<'a> From<&'a mut SmartSocket> for SmartDevice<'a> {
    fn from(value: &'a mut SmartSocket) -> Self {
        Self::Socket(value)
    }
}

impl<'a> From<&'a mut SmartThermometer> for SmartDevice<'a> {
    fn from(value: &'a mut SmartThermometer) -> Self {
        Self::Thermometer(value)
    }
}
