use std::fmt::Display;
use std::str::FromStr;

use crate::{
    smartsocket::{SmartSocket, SmartSocketUpdate},
    smartthermometer::{SmartThermometer, SmartThermometerUpdate},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SmartDeviceParams {
    pub room_id: Option<i64>,
    pub device_type: Option<SmartDeviceType>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum SmartDeviceType {
    Socket,
    Thermometer,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum SmartDeviceUpdate {
    Socket(SmartSocketUpdate),
    Thermometer(SmartThermometerUpdate),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum SmartDevice {
    Socket(SmartSocket),
    Thermometer(SmartThermometer),
}

impl SmartDevice {
    pub fn helper(
        dev: SmartDeviceType,
        id: i64,
        room_id: i64,
        name: String,
        state: bool,
        power: f32,
        temperature: f32,
    ) -> SmartDevice {
        match dev {
            SmartDeviceType::Socket => SmartDevice::Socket(SmartSocket {
                id,
                room_id,
                name,
                state,
                power,
            }),
            SmartDeviceType::Thermometer => SmartDevice::Thermometer(SmartThermometer {
                id,
                room_id,
                name,
                temperature,
            }),
        }
    }

    pub fn get_name(&self) -> &str {
        match self {
            SmartDevice::Socket(s) => s.get_name(),
            SmartDevice::Thermometer(t) => t.get_name(),
        }
    }

    pub fn get_report(&self) -> String {
        match self {
            SmartDevice::Socket(s) => s.get_report(),
            SmartDevice::Thermometer(t) => t.get_report(),
        }
    }

    pub fn get_id(&self) -> i64 {
        match self {
            SmartDevice::Socket(s) => s.id,
            SmartDevice::Thermometer(t) => t.id,
        }
    }

    pub fn get_room_id(&self) -> i64 {
        match self {
            SmartDevice::Socket(s) => s.room_id,
            SmartDevice::Thermometer(t) => t.room_id,
        }
    }
}

impl FromStr for SmartDeviceType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Socket" => Ok(SmartDeviceType::Socket),
            "Thermometer" => Ok(SmartDeviceType::Thermometer),
            _ => Err(()),
        }
    }
}

impl Display for SmartDeviceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SmartDeviceType::Socket => write!(f, "Socket"),
            SmartDeviceType::Thermometer => write!(f, "Thermometer"),
        }
    }
}
