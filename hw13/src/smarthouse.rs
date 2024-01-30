#![allow(dead_code)]
use crate::db::SmartHouseDbApi;
use crate::smartdevice::{SmartDevice, SmartDeviceType, SmartDeviceUpdate};
use crate::smartroom::SmartRoom;
use actix_web::http::StatusCode;
use actix_web::{body::BoxBody, HttpResponse, ResponseError};
use log::{error, info};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Deserialize, Serialize)]
pub struct DelMessage {
    pub rows_deleted: u64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdMessage {
    pub rows_updated: u64,
}

pub struct SmartHouse {
    name: String,
    db: SmartHouseDbApi,
}

#[derive(Error, Debug, PartialEq, Serialize)]
pub enum SmartHouseError {
    #[error("connection error:{0}")]
    NotConnected(String),
    #[error("error during prepare db:{0}")]
    PrepareDB(String),
    #[error("method add_room error:{0}")]
    AddRoom(String),
    #[error("method del_room error:{0}")]
    DelRoom(String),
    #[error("method rooms error:{0}")]
    Rooms(String),
    #[error("RoomNotFound")]
    RoomNotFound,
    #[error("db error:{0}")]
    DBError(String),
    #[error("json error:{0}")]
    JsonError(String),
}

impl From<sqlx::Error> for SmartHouseError {
    fn from(value: sqlx::Error) -> Self {
        SmartHouseError::DBError(value.to_string())
    }
}

impl ResponseError for SmartHouseError {
    fn status_code(&self) -> StatusCode {
        StatusCode::INTERNAL_SERVER_ERROR
    }

    fn error_response(&self) -> actix_web::HttpResponse<BoxBody> {
        HttpResponse::build(self.status_code()).json(self)
    }
}

impl SmartHouse {
    pub async fn new(name: String, db_url: &str) -> Result<Self, SmartHouseError> {
        info!("HTTP-REST API for SmartHouse({})", name);
        let db = SmartHouseDbApi::new(db_url).await?;
        let home = SmartHouse { name, db };
        Ok(home)
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    pub async fn rooms(&self) -> Result<Vec<SmartRoom>, SmartHouseError> {
        let rooms = self.db.select_rooms().await?;
        Ok(rooms)
    }

    pub async fn room_new(&self, name: String) -> Result<SmartRoom, SmartHouseError> {
        let res = self.db.insert_room(name).await?;
        Ok(res)
    }

    pub async fn room_by_id(&self, id: i64) -> Result<SmartRoom, SmartHouseError> {
        let mut res = self.db.select_room_by_id(id).await?;
        res.devices = self.db.select_devices_by_room_id(res.id).await?;
        Ok(res)
    }

    pub async fn room_by_name(&self, name: &String) -> Result<SmartRoom, SmartHouseError> {
        let res = self.db.select_room_by_name(name).await;
        match res {
            Ok(mut room) => {
                room.devices = self.db.select_devices_by_room_id(room.id).await?;
                Ok(room)
            }
            Err(e) => match e {
                sqlx::Error::RowNotFound => Err(SmartHouseError::RoomNotFound),
                _ => Err(e.into()),
            },
        }
    }

    pub async fn room_del_by_id(&self, id: i64) -> Result<DelMessage, SmartHouseError> {
        let res = self.db.delete_room(id).await?;
        Ok(DelMessage { rows_deleted: res })
    }

    pub async fn devices(&self) -> Result<Vec<SmartDevice>, SmartHouseError> {
        let devices = self.db.select_devices().await?;
        Ok(devices)
    }

    pub async fn device_new(
        &self,
        name: String,
        room_id: i64,
        device_type: SmartDeviceType,
    ) -> Result<SmartDevice, SmartHouseError> {
        let res = self.db.insert_device(room_id, name, device_type).await?;
        Ok(res)
    }

    pub async fn device_update(
        &self,
        id: i64,
        upd: SmartDeviceUpdate,
    ) -> Result<UpdMessage, SmartHouseError> {
        let res = self.db.update_device(id, upd).await?;
        Ok(UpdMessage { rows_updated: res })
    }

    pub async fn device_by_id(&self, id: i64) -> Result<SmartDevice, SmartHouseError> {
        let res = self.db.select_device_by_id(id).await?;
        Ok(res)
    }

    pub async fn device_del_by_id(&self, id: i64) -> Result<DelMessage, SmartHouseError> {
        let res = self.db.delete_device(id).await?;
        Ok(DelMessage { rows_deleted: res })
    }
}
