use serde::{Deserialize, Serialize};

use sqlx::{self, SqlitePool};

use crate::smartdevice::{SmartDeviceType, SmartDeviceUpdate};

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct SmartThermometerUpdate {
    pub temperature: Option<f32>,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct SmartThermometer {
    pub id: i64,
    pub room_id: i64,
    pub name: String,
    pub temperature: f32,
}

impl SmartThermometer {
    pub async fn new(db: &SqlitePool, room_id: i64, name: String) -> Result<Self, sqlx::Error> {
        let result = sqlx::query(
            format!(
                "INSERT INTO devices(id, room_id, name, type, temperature) \
                 VALUES (NULL, \"{}\", \"{}\", \"{}\", {});",
                room_id,
                name,
                SmartDeviceType::Thermometer,
                0.0,
            )
            .as_str(),
        )
        .execute(db)
        .await?;
        Ok(SmartThermometer {
            id: result.last_insert_rowid(),
            room_id,
            name,
            temperature: 0.0,
        })
    }

    pub async fn update(
        &mut self,
        db: &SqlitePool,
        upd: SmartDeviceUpdate,
    ) -> Result<u64, sqlx::Error> {
        let upd = match upd {
            SmartDeviceUpdate::Socket(_) => {
                return Err(sqlx::Error::TypeNotFound {
                    type_name: "Trying update Thermometer by SocketUpdate".to_string(),
                });
            }
            SmartDeviceUpdate::Thermometer(t) => t,
        };

        if upd.temperature.is_some() {
            self.temperature = upd.temperature.unwrap();
        }
        let res = sqlx::query(
            format!(
                "UPDATE devices \
                 SET temperature = {} \
                 WHERE id = {};",
                self.temperature, self.id,
            )
            .as_str(),
        )
        .execute(db)
        .await?;
        Ok(res.rows_affected())
    }

    pub fn get_name(&self) -> &str {
        self.name.as_str()
    }

    pub fn get_report(&self) -> String {
        format!(
            "Thermometer({}) temperature: {}",
            self.name, self.temperature
        )
    }
}
