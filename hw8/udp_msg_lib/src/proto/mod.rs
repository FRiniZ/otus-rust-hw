use std::u8;

use prost::Message;

use self::thermomessages::ThermoMsg;

pub mod thermomessages {
    include!(concat!(env!("OUT_DIR"), "/thermomessages.rs"));
}

pub fn thermo_msg_new(temperature: Option<f32>) -> thermomessages::ThermoMsg {
    thermomessages::ThermoMsg { temperature }
}

pub fn thermo_msg_pack(msg: &thermomessages::ThermoMsg) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.reserve(msg.encoded_len());
    msg.encode(&mut buf).unwrap();
    buf
}

pub fn thermo_msg_unpack(buf: &[u8]) -> Result<thermomessages::ThermoMsg, String> {
    let res = ThermoMsg::decode(buf);
    if res.is_err() {
        return Err(format!("Can't decode message:{}", res.unwrap_err()));
    }
    Ok(res.unwrap())
}

pub fn thermo_msg_len(msg: &thermomessages::ThermoMsg) -> u32 {
    msg.encoded_len() as u32
}
