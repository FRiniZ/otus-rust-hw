use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use tokio::net::UdpSocket;
use tokio::sync::Mutex;

use msg_proto_lib::proto::smart_messages::{udp_msg_unpack, SmartMsgType};

#[derive()]
pub struct SmartThermo {
    temperature: Arc<Mutex<f32>>,
}

// impl std::fmt::Display for SmartThermo {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         let temp = self.temperature.lock().await;
//         write!(f, "SmartThermo[temperature:{}]", temp)
//     }
// }

impl SmartThermo {
    pub async fn new(addr: String, finished: Arc<AtomicBool>) -> Self {
        let socket = UdpSocket::bind(addr).await.unwrap();

        let temperature = Arc::new(Mutex::new(0.0));

        let _temperate_clone = Arc::clone(&temperature);
        let _finished_clone = Arc::clone(&finished);

        tokio::spawn(async move {
            loop {
                if _finished_clone.load(Ordering::SeqCst) {
                    return;
                }
                let mut buf = [0; 1024];
                let res = socket.recv(&mut buf).await;
                if res.is_err() {
                    println!("Cant read data{}", res.unwrap_err());
                    break;
                }

                let len = res.unwrap();

                let res = udp_msg_unpack(&buf[0..len]);
                if res.is_err() {
                    println!("Error:{}", res.unwrap_err());
                    _finished_clone.store(true, Ordering::SeqCst);
                    break;
                }

                let msg = res.unwrap();
                {
                    match msg.r#type() {
                        SmartMsgType::None => panic!("Wrong type message"),
                        SmartMsgType::Socket => panic!("Wrong type message"),
                        SmartMsgType::Thermo => {
                            let mut t = _temperate_clone.lock().await;
                            *t = msg.thermo_msg.unwrap().temperature()
                        }
                    }
                }
            }
        });
        Self { temperature }
    }

    pub async fn temperature(&self) -> f32 {
        *self.temperature.lock().await
    }
}
