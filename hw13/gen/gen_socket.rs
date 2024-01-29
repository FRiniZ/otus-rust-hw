use dotenv::dotenv;
use log::{self, info};
use rand::Rng;
use reqwest::Result;
use tokio;

use core::time;
use restapi_smarthouse::{smartdevice::SmartDeviceUpdate, smartsocket::SmartSocketUpdate};
use std::env;

use restapi_smarthouse::smartroom::SmartRoom;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    env_logger::init();

    let room_name = env::var("ROOM").expect("ROOM not set");
    let device_name = env::var("DEVICE").expect("DEVICE not set");
    let url_base = env::var("URL").expect("URL not set");

    let url_room = format!("{}rooms/by_name/{}", url_base, room_name);
    info!("URL: {}", url_room);

    let response = reqwest::get(url_room).await?.text().await?;

    info!("response{:?}", response);

    let room: SmartRoom = serde_json::from_str(response.as_str()).unwrap();
    info!("j: {:?}", room);

    let device = room
        .devices
        .iter()
        .find(|&d| d.get_name() == device_name)
        .unwrap();

    info!("j: {:?}", device);

    let id = match device {
        restapi_smarthouse::smartdevice::SmartDevice::Socket(s) => s.id,
        restapi_smarthouse::smartdevice::SmartDevice::Thermometer(_) => {
            panic!("Found Thermometer. Not Socket")
        }
    };

    info!("id: {:?}", id);

    let client = reqwest::Client::new();

    loop {
        let ssu = SmartDeviceUpdate::Socket(SmartSocketUpdate {
            state: Some(true),
            power: Some(rand::thread_rng().gen_range(0.01..220.00)),
        });
        let url = format!("{}devices/{}", url_base, id);
        let response = client.put(url).json(&ssu).send().await?;
        info!("Response:{:?}", response.text().await);

        tokio::time::sleep(time::Duration::from_secs(1)).await;
        if false {
            break;
        }
    }

    Ok(())
}
