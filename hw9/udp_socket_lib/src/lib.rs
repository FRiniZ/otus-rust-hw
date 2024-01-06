use std::{
    fmt::Display,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::Duration,
};

use rand::Rng;
use tokio::{
    io::Interest,
    net::{TcpSocket, TcpStream},
    sync::Mutex,
    time::sleep,
};

use msg_proto_lib::{
    netmessage::ErrorNetMessage,
    proto::cmd_messages::{cmd_request_unpack, cmd_responce_pack},
};
use msg_proto_lib::{netmessage::NetMessage, proto::cmd_messages::cmd_responce_new};

#[derive()]
pub struct SmartSocket {
    state: bool,
    power: f32,
}

impl SmartSocket {
    fn new(state: bool, power: f32) -> Self {
        Self { state, power }
    }

    pub fn state(&self) -> bool {
        self.state
    }

    pub fn power(&self) -> f32 {
        self.power
    }

    pub fn off(&mut self) {
        self.state = false;
        self.power = 0.0;
    }

    pub fn on(&mut self) {
        self.state = true;
        self.power = rand::thread_rng().gen_range(210.0..240.0);
    }
}

impl Display for SmartSocket {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SmartSocket[state:{}, power:{}]", self.state, self.power)
    }
}

pub struct ServerSmartSocket {
    smartsocket: Arc<Mutex<SmartSocket>>,
}

impl ServerSmartSocket {
    async fn process_conn(
        mut _stream: TcpStream,
        _smartsocket: Arc<Mutex<SmartSocket>>,
    ) -> Result<(), String> {
        println!("Connection from:{}", _stream.peer_addr().unwrap());
        let mut buff: Vec<u8> = Vec::with_capacity(1024);
        let mut reply_queue: Vec<Vec<u8>> = Vec::new();
        loop {
            let _ready = _stream
                .ready(Interest::READABLE | Interest::WRITABLE)
                .await
                .unwrap();

            if _ready.is_writable() {
                if let Some(data) = reply_queue.pop() {
                    let repl = NetMessage::send(&mut _stream, data).await;
                    if repl.is_err() {
                        eprintln!("Error: Can't send responce");
                        return Err("Error: Can't send responce".into());
                    }
                    continue;
                }
            }

            if _ready.is_readable() {
                let res = NetMessage::recv(&mut _stream, &mut buff).await;
                match res {
                    Ok(msg) => {
                        let req = cmd_request_unpack(msg.body_as_bytes());
                        if req.is_ok() {
                            let cmd = req.unwrap();
                            let c = cmd.command.unwrap();
                            let mut ss = _smartsocket.lock().await;
                            let mut responce = String::from("Unknown command");
                            if c.as_str() == "power" {
                                responce = format!("Power consumption:{}, {}", ss.power(), ss);
                            } else if c.as_str() == "off" {
                                ss.off();
                                responce = format!("The socket is turned off:{}", ss);
                            } else if c.as_str() == "on" {
                                ss.on();
                                responce = format!("The socket is turned on:{}", ss);
                            }
                            let cmd_resp = cmd_responce_new(responce);
                            let data = cmd_responce_pack(&cmd_resp);
                            reply_queue.push(data);
                            println!("{}", ss);
                        } else {
                            eprintln!("Error: Can't unpack request");
                            return Err("Error: Can't unpack request".into());
                        }
                    }
                    Err(e) => match e {
                        ErrorNetMessage::IOError(e) => {
                            println!("Close connection with error:{}", e);
                            return Err(format!("Error: {}", e));
                        }
                        ErrorNetMessage::Blocked(_) => continue,
                        ErrorNetMessage::NotReady => continue,
                    },
                }
                continue;
            }

            sleep(Duration::from_millis(100)).await;
        }
    }

    pub async fn listen(addr: String, running: Arc<AtomicBool>) -> Result<(), String> {
        let server = ServerSmartSocket {
            smartsocket: Arc::new(Mutex::new(SmartSocket::new(false, 0.0))),
        };

        let sock = TcpSocket::new_v4().unwrap();
        let addr_parsed = addr.parse().unwrap();
        sock.bind(addr_parsed).unwrap();
        let listener = sock.listen(1024).unwrap();
        tokio::spawn(async move {
            loop {
                let (socket, _) = listener.accept().await.unwrap();
                let smartsocket = Arc::clone(&server.smartsocket);
                tokio::spawn(async move {
                    let _ = ServerSmartSocket::process_conn(socket, smartsocket).await;
                });
            }
        });

        while running.load(Ordering::SeqCst) {
            tokio::time::sleep(Duration::from_millis(100)).await;
        }

        Ok(())
    }
}
