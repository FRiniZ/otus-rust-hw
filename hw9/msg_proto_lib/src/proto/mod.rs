pub mod cmd_messages {
    use std::fmt::Display;

    use prost::Message;

    include!(concat!(env!("OUT_DIR"), "/cmd_messages.rs"));

    impl Display for CmdResponce {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "CmdResponce[responce:{}]", self.responce())
        }
    }

    pub fn cmd_request_new(cmd: String, args: Vec<String>) -> CmdRequest {
        CmdRequest {
            command: Some(cmd),
            arguments: args,
        }
    }

    pub fn cmd_responce_new(resp: String) -> CmdResponce {
        CmdResponce {
            responce: Some(resp),
        }
    }

    pub fn cmd_request_pack(cmd: &CmdRequest) -> Vec<u8> {
        let mut buf = Vec::new();
        buf.reserve(cmd.encoded_len());
        cmd.encode(&mut buf).unwrap();
        buf
    }

    pub fn cmd_responce_pack(cmd: &CmdResponce) -> Vec<u8> {
        let mut buf = Vec::new();
        buf.reserve(cmd.encoded_len());
        cmd.encode(&mut buf).unwrap();
        buf
    }

    pub fn cmd_request_unpack(buf: &[u8]) -> Result<CmdRequest, String> {
        let res = CmdRequest::decode(buf);
        if res.is_err() {
            return Err(format!("Can't decode message:{}", res.unwrap_err()));
        }
        Ok(res.unwrap())
    }

    pub fn cmd_responce_unpack(buf: &[u8]) -> Result<CmdResponce, String> {
        let res = CmdResponce::decode(buf);
        if res.is_err() {
            return Err(format!("Can't decode message:{}", res.unwrap_err()));
        }
        Ok(res.unwrap())
    }
}

pub mod smart_messages {
    use prost::Message;

    include!(concat!(env!("OUT_DIR"), "/smart_messages.rs"));

    pub fn udp_msg_new(
        r#type: SmartMsgType,
        thermo: Option<ThermoMsg>,
        socket: Option<SocketMsg>,
    ) -> Result<SmartMsg, String> {
        match r#type {
            SmartMsgType::None => {
                return Err("SmartMsgType can't be None".to_string());
            }
            SmartMsgType::Socket => {
                if socket.is_none() {
                    return Err("Socket field is empty".to_string());
                }
            }

            SmartMsgType::Thermo => {
                if thermo.is_none() {
                    return Err("Thermo field is empty".to_string());
                }
            }
        };

        Ok(SmartMsg {
            r#type: r#type as i32,
            thermo_msg: thermo,
            socket_msg: socket,
        })
    }
    pub fn thermo_msg_new(temperature: Option<f32>) -> SmartMsg {
        SmartMsg {
            r#type: SmartMsgType::Thermo as i32,
            thermo_msg: Some(ThermoMsg { temperature }),
            socket_msg: None,
        }
    }

    pub fn socket_msg_new(state: Option<bool>, power: Option<f32>) -> SmartMsg {
        SmartMsg {
            r#type: SmartMsgType::Socket as i32,
            thermo_msg: None,
            socket_msg: Some(SocketMsg { state, power }),
        }
    }

    pub fn udp_msg_pack(msg: &SmartMsg) -> Vec<u8> {
        let mut buf = Vec::new();
        buf.reserve(msg.encoded_len());
        msg.encode(&mut buf).unwrap();
        buf
    }
    pub fn udp_msg_unpack(buf: &[u8]) -> Result<SmartMsg, String> {
        let res = SmartMsg::decode(buf);
        if res.is_err() {
            return Err(format!("Can't decode message:{}", res.unwrap_err()));
        }
        Ok(res.unwrap())
    }
}
