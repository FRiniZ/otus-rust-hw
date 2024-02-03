use serde::{Deserialize, Serialize};

use sqlx::{self, SqlitePool};

use crate::smartdevice::{SmartDeviceType, SmartDeviceUpdate};

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct SmartSocketUpdate {
    pub state: Option<bool>,
    pub power: Option<f32>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct SmartSocket {
    pub id: i64,
    pub room_id: i64,
    pub name: String,
    pub state: bool,
    pub power: f32,
}

impl SmartSocket {
    pub async fn new(db: &SqlitePool, room_id: i64, name: String) -> Result<Self, sqlx::Error> {
        let result = sqlx::query(
            format!(
                "INSERT INTO devices(id, room_id, name, type, state, power) \
                 VALUES (NULL, \"{}\", \"{}\", \"{}\", {}, {});",
                room_id,
                name,
                SmartDeviceType::Socket,
                false,
                0.0,
            )
            .as_str(),
        )
        .execute(db)
        .await?;
        Ok(SmartSocket {
            id: result.last_insert_rowid(),
            room_id,
            name,
            state: false,
            power: 0.0,
        })
    }

    pub async fn update(
        &mut self,
        db: &SqlitePool,
        upd: SmartDeviceUpdate,
    ) -> Result<u64, sqlx::Error> {
        let upd = match upd {
            SmartDeviceUpdate::Socket(u) => u,
            SmartDeviceUpdate::Thermometer(_) => {
                return Err(sqlx::Error::TypeNotFound {
                    type_name: "Trying update Socket by ThermometerUpdate".to_string(),
                });
            }
        };

        if upd.state.is_some() {
            self.state = upd.state.unwrap();
        }

        if upd.power.is_some() {
            self.power = upd.power.unwrap();
        }

        let res = sqlx::query(
            format!(
                "UPDATE devices \
                 SET state = {}, power = {} \
                 WHERE id = {};",
                self.state, self.power, self.id,
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

    fn get_state(&self) -> &str {
        if self.state {
            "On"
        } else {
            "Off"
        }
    }
    pub fn get_report(&self) -> String {
        format!(
            "Socket({}) state:{} power: {}",
            self.name,
            self.get_state(),
            self.power
        )
    }
}
