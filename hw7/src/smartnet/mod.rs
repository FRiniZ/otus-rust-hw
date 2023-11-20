pub mod message;
pub mod netmessage;

use crate::smarthouse::{OwningDeviceInfoProvider, SmartHouse};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::sync::{Arc, Mutex};

use std::{fmt, thread};

use self::message::Message;

struct SmartProvider {
    provider: OwningDeviceInfoProvider,
    smarthouse: SmartHouse,
}

pub struct SmartService {
    tcp: TcpListener,
    smartprovider: Arc<Mutex<SmartProvider>>,
}

fn handle_request(
    request: Message,
    smartprovider: Arc<Mutex<SmartProvider>>,
    stream: &mut TcpStream,
) -> Result<(), String> {
    if request.report.is_some() {
        println!("Client request report");
        let sp = smartprovider.lock().unwrap();
        let report = sp.smarthouse.create_report(&sp.provider);
        println!("Prepared report:\n{report}");
        let repl = Message::send(
            stream,
            Message {
                report: Some(report),
                room: None,
                device: None,
                cmd: None,
            },
        );
        if repl.is_err() {
            eprintln!("Can't send reply with report");
            return Err("Can't send reply with report".to_string());
        }
        return Ok(());
    };

    if let Some(cmd) = request.cmd {
        println!("Client request cmd:{}", cmd);
        if let Some(room) = request.room {
            if let Some(device) = request.device {
                let mut sp = smartprovider.lock().unwrap();
                let report = sp.provider.device_cmd(&room, &device, &cmd);
                let repl = Message::send(
                    stream,
                    Message {
                        report: Some(report),
                        room: None,
                        device: None,
                        cmd: None,
                    },
                );
                if repl.is_err() {
                    eprintln!("Can't send reply with report");
                    return Err("Can't send reply with report".to_string());
                }
            };
        };
    };

    Err("Undefine behaviour".to_string())
}

fn handle_connection(smartprovider: Arc<Mutex<SmartProvider>>, mut stream: TcpStream) {
    let req = Message::recv(&mut stream);
    if let Ok(msg) = req {
        let _ = handle_request(msg, smartprovider, &mut stream);
    }
    stream
        .shutdown(Shutdown::Both)
        .expect("shutdown call failed");
    println!("Client disconnected:{}", stream.peer_addr().unwrap());
}

impl SmartService {
    pub fn new(
        smarthouse: SmartHouse,
        provider: OwningDeviceInfoProvider,
        addr: String,
    ) -> std::io::Result<Self> {
        let tcp = TcpListener::bind(addr)?;
        Ok(Self {
            tcp,
            smartprovider: Arc::new(Mutex::new(SmartProvider {
                provider,
                smarthouse,
            })),
        })
    }

    pub fn serve(&self) {
        for stream in self.tcp.incoming() {
            match stream {
                Ok(stream) => {
                    println!("Client connected:{}", stream.peer_addr().unwrap());
                    let smartprovider = self.smartprovider.clone();
                    thread::spawn(move || handle_connection(smartprovider, stream));
                }
                Err(e) => {
                    eprintln!("Failed connection:{}", e);
                }
            }
        }
    }
}

impl fmt::Debug for SmartService {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SmartService")
    }
}
