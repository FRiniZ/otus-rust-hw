use crate::smartdevice::SmartDevice;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct SmartRoom {
    pub id: i64,
    pub name: String,
    pub devices: Vec<SmartDevice>,
}
