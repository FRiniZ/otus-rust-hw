#![allow(dead_code)]
mod db;
mod router;
mod smartdevice;
mod smarthouse;
mod smartreport;
mod smartroom;
mod smartsocket;
mod smartthermometer;

use std::env;

use crate::smartdevice::SmartDevice;
use crate::smartsocket::SmartSocket;

use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use log::{self, info};
use tokio::sync::Mutex;

use smarthouse::SmartHouse;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let host = env::var("HOST").expect("HOST not set");
    let port = env::var("PORT").expect("PORT not set");
    let db_url = env::var("DB_URL").expect("DB_URL not set");

    let sr = SmartDevice::Socket(SmartSocket::default());
    let j = serde_json::to_string(&sr);

    info!("JSON:{:?}", j);

    let smarthouse = SmartHouse::new(String::from("Sweet Дом"), db_url.as_str())
        .await
        .unwrap();

    info!("Starting server[{}:{}]", host, port);

    let _opaque = web::Data::new(Mutex::new(smarthouse));
    HttpServer::new(move || {
        App::new()
            .app_data(_opaque.clone())
            .service(router::index)
            .service(router::report)
            .service(router::rooms)
            .service(router::room_new)
            .service(router::room_by_id)
            .service(router::room_devices)
            .service(router::room_del_by_id)
            .service(router::devices)
            .service(router::device_new)
            .service(router::device_update)
            .service(router::device_by_id)
            .service(router::device_del_by_id)
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}
