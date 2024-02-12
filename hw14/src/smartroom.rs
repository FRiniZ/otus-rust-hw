use crate::smartdevice::SmartDevice;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct SmartRoom {
    pub id: i64,
    pub name: String,
    pub devices: Vec<SmartDevice>,
}
