use std::fmt::Display;
use std::{io, u32};

use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

#[derive(Debug)]
pub enum ErrorNetMessage {
    IOError(String),
    Blocked(()),
    NotReady,
}

pub enum NetMessageState {
    WaitSize(u32),
    WaitBody(Vec<u8>),
    Ready(NetMessage),
}

#[derive(Debug)]
pub struct NetMessage {
    size: u32,
    body: Vec<u8>,
}

impl Display for NetMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "NetMessage[size: {}, body_len:{}]",
            self.size,
            self.body.len()
        )
    }
}

impl NetMessage {
    pub fn new(size: u32, body: Vec<u8>) -> Self {
        Self { size, body }
    }

    pub fn clean(&mut self) {
        self.size = 0;
        self.body.clear();
    }

    pub async fn recv(
        stream: &mut TcpStream,
        buff: &mut Vec<u8>,
    ) -> Result<NetMessage, ErrorNetMessage> {
        //Read size
        let mut buf_size = [0_u8; 1024];

        match stream.try_read(&mut buf_size) {
            Ok(0) => {
                return Err(ErrorNetMessage::IOError("lost connection".to_string()));
            }
            Ok(n) => {
                buff.extend_from_slice(&buf_size[0..n]);
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                return Err(ErrorNetMessage::Blocked(()))
            }
            Err(e) => return Err(ErrorNetMessage::IOError(format!("Error: {}", e))),
        }

        if buff.len() < 4 {
            return Err(ErrorNetMessage::NotReady);
        }

        let size = u32::from_be_bytes(buff.get(0..4).unwrap().try_into().unwrap());

        let full_size: usize = size as usize + 4_usize;

        if buff.len() < full_size {
            return Err(ErrorNetMessage::NotReady);
        }

        let v_size: Vec<u8> = buff.drain(0..4).collect();
        let v_body = buff.drain(0..size as usize).collect();

        let size = u32::from_be_bytes(v_size.get(0..4).unwrap().try_into().unwrap());

        let msg = NetMessage::new(size, v_body);

        buff.clear();

        Ok(msg)
    }

    //TODO rewrite to use try_write
    pub async fn send(stream: &mut TcpStream, data: Vec<u8>) -> Result<(), String> {
        let msg = NetMessage::new(data.len() as u32, data);
        let ret = stream.write_all(&msg.size_as_bytes()).await;
        if ret.is_err() {
            return Err(format!("Can't send size of message:{}", ret.unwrap_err()));
        }
        let ret = stream.write_all(msg.body_as_bytes()).await;
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
