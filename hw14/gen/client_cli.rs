use clap::{Parser, Subcommand, ValueEnum};
use dotenv::dotenv;
use reqwest::{self, StatusCode};
use restapi_smarthouse::smartdevice::{SmartDevice, SmartDeviceParams, SmartDeviceType};
use restapi_smarthouse::smartreport::{SmartReport, SmartReportParams};
use std::env;
use std::error::Error;

use restapi_smarthouse::smartroom::SmartRoom;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Debug, Clone, ValueEnum)]
enum DeviceType {
    Socket,
    Thermometer,
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    /// List all rooms
    ListRooms,
    ///Create new room
    RoomAdd {
        /// Room name - must be unique in house
        name: String,
    },
    ///Delete room
    RoomDel {
        /// Room id - which will removed
        id: i64,
    },
    ///Create new device in room
    DeviceAdd {
        /// Room id - where the device will be added
        room_id: i64,
        /// Device name - must be unique in room
        name: String,
        /// Device type
        device_type: DeviceType,
    },
    ///Delete device
    DeviceDel {
        ///Device's Id
        id: i64,
    },
    ///Get information from about room by id or name
    GetRoom {
        // Optional Room's Id
        id: i64,
    },
    ///Get information about device
    GetDevice {
        /// Device's ID
        id: i64,
    },
    ///Report
    Report {
        /// Json example: {"request":[{"room": "room1","device": "socket1"}, {"room": "room2", "device": "thermo1"}]}
        json: String,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let _cli = Cli::parse();

    dotenv().ok();
    env_logger::init();
    let url_base = env::var("URL").expect("URL not set");

    match _cli.cmd {
        Commands::ListRooms => list_rooms(url_base).await,
        Commands::RoomAdd { name } => room_add(url_base, name).await,
        Commands::RoomDel { id } => room_del(url_base, id).await,
        Commands::DeviceAdd {
            room_id,
            name,
            device_type,
        } => device_add(url_base, room_id, name, device_type).await,
        Commands::DeviceDel { id } => device_del(url_base, id).await,
        Commands::GetRoom { id } => get_room(url_base, id).await,
        Commands::GetDevice { id } => get_device(url_base, id).await,
        Commands::Report { json } => get_report(url_base, json).await,
    }
}

async fn list_rooms(url_base: String) -> Result<(), Box<dyn Error>> {
    let url = format!("{}rooms", url_base);

    let response = reqwest::get(url).await?.text().await?;
    let rooms: Vec<SmartRoom> = serde_json::from_str(response.as_str())?;
    for room in rooms.iter() {
        println!("Room:(id: {} name:{})", room.id, room.name);
        for dev in room.devices.iter() {
            println!(
                "\t Device:(id: {}, name: {}, Report: {})",
                dev.get_id(),
                dev.get_name(),
                dev.get_report(),
            );
        }
    }
    Ok(())
}

async fn room_add(url_base: String, name: String) -> Result<(), Box<dyn Error>> {
    let client = reqwest::Client::new();
    let url = format!("{}rooms/{}", url_base, name);

    let response = client.post(url).send().await?;
    let code = response.status();

    if code != StatusCode::CREATED {
        let error_text = response.text().await?;
        return Err(error_text.into());
    }
    let text = response.text().await?;
    let room: SmartRoom = serde_json::from_str(text.as_str())?;
    println!(
        "Room created:(id: {}, name: {}, devices: {:?})",
        room.id, room.name, room.devices
    );

    Ok(())
}

async fn room_del(url_base: String, id: i64) -> Result<(), Box<dyn Error>> {
    let client = reqwest::Client::new();
    let url = format!("{}rooms/{}", url_base, id);

    let response = client.delete(url).send().await?;
    let code = response.status();

    if code != StatusCode::OK {
        let error_text = response.text().await?;
        return Err(error_text.into());
    }

    let text = response.text().await?;
    println!("Room has been deleted: {}", text);

    Ok(())
}

async fn device_add(
    url_base: String,
    room_id: i64,
    name: String,
    dev_type: DeviceType,
) -> Result<(), Box<dyn Error>> {
    let client = reqwest::Client::new();
    let url = format!("{}devices/{}", url_base, name);

    let device_type = match dev_type {
        DeviceType::Socket => SmartDeviceType::Socket,
        DeviceType::Thermometer => SmartDeviceType::Thermometer,
    };

    let sdu = SmartDeviceParams {
        room_id: Some(room_id),
        device_type: Some(device_type),
    };

    let response = client.post(url).json(&sdu).send().await?;
    let code = response.status();

    if code != StatusCode::OK {
        let error_text = response.text().await?;
        return Err(error_text.into());
    }

    let text = response.text().await?;

    let device: SmartDevice = serde_json::from_str(text.as_str())?;
    println!(
        "Device created:(id: {}, room_id: {}, name: {}, report: {})",
        device.get_id(),
        device.get_room_id(),
        device.get_name(),
        device.get_report(),
    );

    Ok(())
}

async fn device_del(url_base: String, id: i64) -> Result<(), Box<dyn Error>> {
    let client = reqwest::Client::new();
    let url = format!("{}devices/{}", url_base, id);

    let response = client.delete(url).send().await?;
    let code = response.status();

    if code != StatusCode::OK {
        let error_text = response.text().await?;
        return Err(error_text.into());
    }

    let text = response.text().await?;
    println!("Device has been deleted: {}", text);

    Ok(())
}

async fn get_room(url_base: String, id: i64) -> Result<(), Box<dyn Error>> {
    let url = format!("{}rooms/{}", url_base, id);

    let response = reqwest::get(url).await?;
    let code = response.status();

    if code != StatusCode::OK {
        let error_text = response.text().await?;
        return Err(error_text.into());
    }

    let text = response.text().await?;
    let room: SmartRoom = serde_json::from_str(text.as_str())?;
    println!("Room:(id: {} name:{})", room.id, room.name);

    for dev in room.devices.iter() {
        println!(
            "\t Device:(id: {}, name: {}, Report: {})",
            dev.get_id(),
            dev.get_name(),
            dev.get_report(),
        );
    }
    Ok(())
}

async fn get_device(url_base: String, id: i64) -> Result<(), Box<dyn Error>> {
    let url = format!("{}devices/{}", url_base, id);

    let response = reqwest::get(url).await?;
    let code = response.status();

    if code != StatusCode::OK {
        let error_text = response.text().await?;
        return Err(error_text.into());
    }

    let text = response.text().await?;
    let device: SmartDevice = serde_json::from_str(text.as_str())?;
    println!(
        "Device: (id: {}, room_id: {}, name: {}, report: {})",
        device.get_id(),
        device.get_room_id(),
        device.get_name(),
        device.get_report(),
    );
    Ok(())
}

async fn get_report(url_base: String, json: String) -> Result<(), Box<dyn Error>> {
    let client = reqwest::Client::new();
    let url = format!("{}report", url_base);

    let j: SmartReportParams = serde_json::from_str(json.as_str())?;

    let response = client.post(url).json(&j).send().await?;
    let code = response.status();

    if code != StatusCode::OK {
        let error_text = response.text().await?;
        return Err(error_text.into());
    }

    let text = response.text().await?;
    let _report: SmartReport = serde_json::from_str(text.as_str())?;

    for rep in _report.reports.iter() {
        println!(
            "Request: (room:{}, device:{}) Result:({})",
            rep.room, rep.device, rep.report
        );
    }
    Ok(())
}
