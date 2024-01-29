use std::ops::Deref;

use crate::{
    smartdevice::{SmartDeviceParams, SmartDeviceUpdate},
    smarthouse::{SmartHouse, SmartHouseError},
    smartreport::{SmartReport, SmartReportParams, SmartReportResponce},
};
use tokio::sync::Mutex;

use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use log::{self, debug, info};

#[get("/")]
async fn index(data: web::Data<Mutex<SmartHouse>>) -> impl Responder {
    info!("get /");
    let text = format!(
        "Example REST API of SmartHouse2[{}]",
        data.lock().await.name()
    );
    HttpResponse::Ok()
        .content_type("text/plain; charset=utf-8")
        .body(text)
}

#[get("/rooms")]
async fn rooms(data: web::Data<Mutex<SmartHouse>>) -> Result<HttpResponse, SmartHouseError> {
    debug!("get /rooms");
    let home = data.lock().await;
    let res = home.rooms().await?;
    Ok(HttpResponse::Ok()
        .content_type("text/plain; charset=utf-8")
        .json(res))
}

#[get("/rooms/{id}")]
async fn room_by_id(
    data: web::Data<Mutex<SmartHouse>>,
    id: web::Path<i64>,
) -> Result<HttpResponse, SmartHouseError> {
    debug!("get /rooms/{}", id);
    let home = data.lock().await;
    let res = home.room_by_id(*id).await?;
    Ok(HttpResponse::Ok()
        .content_type("text/plain; charset=utf-8")
        .json(res))
}

#[get("/rooms/by_name/{name}")]
async fn room_by_name(
    data: web::Data<Mutex<SmartHouse>>,
    name: web::Path<String>,
) -> Result<HttpResponse, SmartHouseError> {
    debug!("get /rooms/by_name/{}", name);
    let home = data.lock().await;
    let res = home.room_by_name(&name.deref()).await?;
    Ok(HttpResponse::Ok()
        .content_type("text/plain; charset=utf-8")
        .json(res))
}

#[get("/rooms/{id}/devices")]
async fn room_devices(
    data: web::Data<Mutex<SmartHouse>>,
    id: web::Path<i64>,
) -> Result<HttpResponse, SmartHouseError> {
    debug!("get /rooms/{}", id);
    let home = data.lock().await;
    let res = home.room_by_id(*id).await?;
    Ok(HttpResponse::Ok()
        .content_type("text/plain; charset=utf-8")
        .json(res.devices))
}

#[post("/rooms/{name}")]
async fn room_new(
    data: web::Data<Mutex<SmartHouse>>,
    name: web::Path<String>,
) -> Result<HttpResponse, SmartHouseError> {
    debug!("get /add_room/{}", name);
    let home = data.lock().await;
    let res = home.room_new(name.clone()).await?;
    Ok(HttpResponse::Created()
        .content_type("text/plain; charset=utf-8")
        .json(res))
}

#[delete("/rooms/{id}")]
async fn room_del_by_id(
    data: web::Data<Mutex<SmartHouse>>,
    id: web::Path<i64>,
) -> Result<HttpResponse, SmartHouseError> {
    debug!("delete /rooms/by_id/{}", id);
    let home = data.lock().await;
    let res = home.room_del_by_id(*id).await?;
    Ok(HttpResponse::Accepted().json(res))
}

#[get("/devices")]
async fn devices(data: web::Data<Mutex<SmartHouse>>) -> Result<HttpResponse, SmartHouseError> {
    debug!("get /devices");
    let home = data.lock().await;
    let res = home.devices().await?;
    Ok(HttpResponse::Ok()
        .content_type("text/plain; charset=utf-8")
        .json(res))
}

#[get("/devices/{id}")]
async fn device_by_id(
    data: web::Data<Mutex<SmartHouse>>,
    id: web::Path<i64>,
) -> Result<HttpResponse, SmartHouseError> {
    debug!("get /devices/{}", id);
    let home = data.lock().await;
    let res = home.device_by_id(*id).await?;
    Ok(HttpResponse::Ok()
        .content_type("text/plain; charset=utf-8")
        .json(res))
}

#[post("/devices/{name}")]
async fn device_new(
    data: web::Data<Mutex<SmartHouse>>,
    name: web::Path<String>,
    json: web::Json<SmartDeviceParams>,
) -> Result<HttpResponse, SmartHouseError> {
    debug!("post /devices/{} json:{:?}", name, json);
    if json.room_id.is_none() {
        return Err(SmartHouseError::JsonError(String::from(
            "miss room_id field",
        )));
    }
    if json.device_type.is_none() {
        return Err(SmartHouseError::JsonError(String::from(
            "miss device_type field",
        )));
    }
    let room_id = json.room_id.unwrap();
    let device_type = json.device_type.unwrap();
    let home = data.lock().await;
    let res = home.device_new(name.clone(), room_id, device_type).await?;
    Ok(HttpResponse::Ok()
        .content_type("text/plain; charset=utf-8")
        .json(res))
}

#[delete("/devices/{id}")]
async fn device_del_by_id(
    data: web::Data<Mutex<SmartHouse>>,
    id: web::Path<i64>,
) -> Result<HttpResponse, SmartHouseError> {
    debug!("delete /devices/{}", id);
    let home = data.lock().await;
    let res = home.device_del_by_id(*id).await?;
    Ok(HttpResponse::Accepted().json(res))
}

#[put("/devices/{id}")]
async fn device_update(
    data: web::Data<Mutex<SmartHouse>>,
    id: web::Path<i64>,
    device: web::Json<SmartDeviceUpdate>,
) -> Result<HttpResponse, SmartHouseError> {
    debug!("put /devices/{} json:{:?}", id, device);

    let home = data.lock().await;
    let res = home.device_update(*id, *device).await?;
    Ok(HttpResponse::Ok()
        .content_type("text/plain; charset=utf-8")
        .json(res))
}

#[post("/report")]
async fn report(
    data: web::Data<Mutex<SmartHouse>>,
    json: web::Json<SmartReportParams>,
) -> Result<HttpResponse, SmartHouseError> {
    debug!("post /report json:{:?}", json);

    let mut result = SmartReport::default();
    for req in json.request.iter() {
        info!("req:{:?}", req);
        let home = data.lock().await;
        let find_room = home.room_by_name(&req.room).await;
        let room = match find_room {
            Ok(r) => r,
            Err(e) => match e {
                SmartHouseError::RoomNotFound => {
                    result.reports.push(SmartReportResponce {
                        room: String::clone(&req.room),
                        device: String::clone(&req.device),
                        report: "Room not found".to_string(),
                    });
                    continue;
                }
                _ => {
                    return Ok(HttpResponse::InternalServerError()
                        .content_type("text/plain; charset=utf-8")
                        .json(e))
                }
            },
        };

        let device = room.devices.iter().find(|&d| d.get_name() == req.device);
        let report = match device {
            Some(d) => d.get_report(),
            None => format!("Device({}) not found in Room({})", req.device, req.room),
        };

        result.reports.push(SmartReportResponce {
            room: String::clone(&req.room),
            device: String::clone(&req.device),
            report,
        });
    }

    Ok(HttpResponse::Ok()
        .content_type("text/plain; charset=utf-8")
        .json(result))
}
