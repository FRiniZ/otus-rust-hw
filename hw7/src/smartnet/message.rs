use core::fmt;
use std::net::TcpStream;
use std::str;

use serde_json;

use super::netmessage;
use super::netmessage::NetMessage;

use serde::Deserialize;
use serde::Serialize;
#[derive(Serialize, Deserialize)]
pub struct Message {
    pub report: Option<String>,
    pub room: Option<String>,
    pub device: Option<String>,
    pub cmd: Option<String>,
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let json = serde_json::to_string(self).unwrap();
        write!(f, "{}", json)
    }
}

impl Message {
    pub fn new(
        report: Option<String>,
        room: Option<String>,
        device: Option<String>,
        cmd: Option<String>,
    ) -> Self {
        Self {
            report,
            room,
            device,
            cmd,
        }
    }

    fn to_vec(&self) -> Vec<u8> {
        let json = serde_json::to_string(self).unwrap();
        json.as_bytes().to_vec()
    }

    pub fn recv(stream: &mut TcpStream) -> Result<Message, String> {
        let nmsg = netmessage::NetMessage::recv(stream)?;
        let json = match str::from_utf8(nmsg.body_as_bytes()) {
            Ok(s) => s,
            Err(e) => {
                return Err(format!(
                    "Can't convert NetMessage.Data to UTF8-String:{}",
                    e
                ));
            }
        };

        let msg: Message = serde_json::from_str(json).unwrap();
        Ok(msg)
    }

    pub fn send(stream: &mut TcpStream, msg: Message) -> Result<(), String> {
        NetMessage::send(stream, msg.to_vec())?;
        Ok(())
    }
}
