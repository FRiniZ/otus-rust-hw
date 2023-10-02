pub mod ssocket;
pub mod sthermometer;

use ssocket::SmartSocket;
use sthermometer::SmartThermometer;

fn main() {
    let _socket = SmartSocket::new(String::from("Very smart socket"));
    let _thermometer = SmartThermometer::new(-60.0, 60.0);
}
