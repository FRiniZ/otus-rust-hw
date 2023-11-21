use std::{
    io::{Read, Write},
    net::TcpStream,
};

pub struct NetMessage {
    size: u32,
    body: Vec<u8>,
}

impl NetMessage {
    pub fn new(size: u32, body: Vec<u8>) -> Self {
        Self { size, body }
    }

    pub fn recv(stream: &mut TcpStream) -> Result<NetMessage, String> {
        //Read size
        let mut buf_size = [0_u8; 4];
        let mut msg = Self {
            size: 0,
            body: vec![],
        };

        let ret = stream.read_exact(&mut buf_size);
        if ret.is_err() {
            return Err(format!("Can't read size:{}", ret.unwrap_err()));
        }

        msg.size = u32::from_be_bytes(buf_size);

        let mut buf_body = vec![0; msg.size as usize];
        let ret = stream.read_exact(&mut buf_body);
        if ret.is_err() {
            return Err(format!("Can't read body:{}", ret.unwrap_err()));
        }

        msg.body = buf_body;

        Ok(msg)
    }

    pub fn send(stream: &mut TcpStream, data: Vec<u8>) -> Result<(), String> {
        let msg = NetMessage::new(data.len() as u32, data);
        let ret = stream.write_all(&msg.size_as_bytes());
        if ret.is_err() {
            return Err(format!("Can't send size of message:{}", ret.unwrap_err()));
        }
        let ret = stream.write_all(msg.body_as_bytes());
        if ret.is_err() {
            return Err(format!("Can't send body of message:{}", ret.unwrap_err()));
        }
        Ok(())
    }

    pub fn size_as_bytes(&self) -> [u8; 4] {
        self.size.to_be_bytes()
    }

    pub fn body_as_bytes(&self) -> &[u8] {
        self.body.as_slice()
    }
}
