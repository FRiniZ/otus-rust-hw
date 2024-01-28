use std::str::FromStr;

use crate::smartdevice::{SmartDevice, SmartDeviceType, SmartDeviceUpdate};
use crate::smartroom::SmartRoom;
use crate::smartsocket::SmartSocket;
use crate::smartthermometer::SmartThermometer;
use log::info;
use sqlx::Row;
use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool};

const SQL_CREATE_TABLE_ROOMS: &str = "    \
        CREATE TABLE IF NOT EXISTS rooms ( \
                                          id INTEGER PRIMARY KEY AUTOINCREMENT,\
                                          name VARCHAR(255) NOT NULL UNIQUE);";

const SQL_CREATE_TABLE_DEVICES: &str = "\
    CREATE TABLE IF NOT EXISTS devices ( \
                                          id INTEGER PRIMARY KEY NOT NULL,\
                                          room_id INTEGER NOT NULL,\
                                          name VARCHAR(255) NOT NULL,\
                                          type VARCHAR(255) NOT NULL,\
                                          state BOOLEAN,\
                                          power REAL,\
                                          temperature REAL,\
                                          FOREIGN KEY (room_id) REFERENCES rooms (id),\
                                          UNIQUE (room_id, name));";

pub struct SmartHouseDbApi {
    pub db: SqlitePool,
}

impl SmartHouseDbApi {
    async fn connect(url: &str) -> Result<SqlitePool, sqlx::Error> {
        if !Sqlite::database_exists(url).await.unwrap_or(false) {
            info!("Createing database {}", url);
            match Sqlite::create_database(url).await {
                Ok(_) => info!("Create database success"),
                Err(e) => return Err(e),
            }
        } else {
            info!("Database already exists");
        }
        let db = SqlitePool::connect(url).await.unwrap();
        info!("Connected database");
        Ok(db)
    }

    async fn checktables(&self) -> Result<(), sqlx::Error> {
        let result = sqlx::query(SQL_CREATE_TABLE_ROOMS)
            .execute(&self.db)
            .await
            .unwrap();
        info!("Create rooms table result: {:?}", result);

        let result = sqlx::query(SQL_CREATE_TABLE_DEVICES)
            .execute(&self.db)
            .await
            .unwrap();
        info!("Create rooms table result: {:?}", result);

        Ok(())
    }

    pub async fn new(db_url: &str) -> Result<Self, sqlx::Error> {
        let pool = SmartHouseDbApi::connect(db_url).await?;
        let db = SmartHouseDbApi { db: pool };
        db.checktables().await.unwrap();
        Ok(db)
    }

    pub async fn insert_room(&self, name: String) -> Result<SmartRoom, sqlx::Error> {
        let result = sqlx::query(
            format!("INSERT INTO rooms(id, name) VALUES (NULL, \"{}\")", name).as_str(),
        )
        .execute(&self.db)
        .await;
        if result.is_err() {
            return Err(result.unwrap_err());
        }

        Ok(SmartRoom {
            id: result.unwrap().last_insert_rowid(),
            name,
            devices: vec![],
        })
    }

    pub async fn delete_room(&self, id: i64) -> Result<u64, sqlx::Error> {
        let sql = format!("DELETE FROM rooms WHERE id = {};", id);
        let result = sqlx::query(sql.as_str()).execute(&self.db).await?;
        Ok(result.rows_affected())
    }

    pub async fn select_room_by_name(&self, name: &String) -> Result<SmartRoom, sqlx::Error> {
        let row =
            sqlx::query(format!("SELECT id, name FROM rooms WHERE name = \"{}\";", name).as_str())
                .fetch_one(&self.db)
                .await?;

        let id = row.get::<i64, &str>("id");
        let name = row.get::<String, &str>("name");
        let room = SmartRoom {
            id,
            name,
            devices: vec![],
        };
        Ok(room)
    }

    pub async fn select_room_by_id(&self, id: i64) -> Result<SmartRoom, sqlx::Error> {
        let row = sqlx::query(format!("SELECT id, name FROM rooms WHERE id = {};", id).as_str())
            .fetch_one(&self.db)
            .await?;
        let id = row.get::<i64, &str>("id");
        let name = row.get::<String, &str>("name");
        let room = SmartRoom {
            id,
            name,
            devices: vec![],
        };

        Ok(room)
    }

    pub async fn select_rooms(&self) -> Result<Vec<SmartRoom>, sqlx::Error> {
        let mut rooms: Vec<SmartRoom> = Vec::new();
        let result = sqlx::query("SELECT id, name FROM rooms;")
            .fetch_all(&self.db)
            .await?;

        for (_, row) in result.iter().enumerate() {
            let id = row.get::<i64, &str>("id");
            let name = row.get::<String, &str>("name");
            let devices = self.select_devices_by_room_id(id).await?;

            let room = SmartRoom { id, name, devices };
            rooms.push(room);
        }
        Ok(rooms)
    }

    pub async fn select_devices(&self) -> Result<Vec<SmartDevice>, sqlx::Error> {
        let mut devices: Vec<SmartDevice> = Vec::new();
        let result = sqlx::query("SELECT * FROM devices;")
            .fetch_all(&self.db)
            .await?;

        for (_, row) in result.iter().enumerate() {
            let dev = SmartDevice::helper(
                SmartDeviceType::from_str(row.get::<&str, &str>("type")).unwrap(),
                row.get::<i64, &str>("id"),
                row.get::<i64, &str>("room_id"),
                row.get::<String, &str>("name"),
                row.get::<bool, &str>("state"),
                row.get::<f32, &str>("power"),
                row.get::<f32, &str>("temperature"),
            );
            devices.push(dev);
        }
        Ok(devices)
    }

    pub async fn select_devices_by_room_id(
        &self,
        room_id: i64,
    ) -> Result<Vec<SmartDevice>, sqlx::Error> {
        let mut devices: Vec<SmartDevice> = Vec::new();
        let result =
            sqlx::query(format!("SELECT * FROM devices WHERE room_id = {};", room_id).as_str())
                .fetch_all(&self.db)
                .await?;

        for (_, row) in result.iter().enumerate() {
            let dev = SmartDevice::helper(
                SmartDeviceType::from_str(row.get::<&str, &str>("type")).unwrap(),
                row.get::<i64, &str>("id"),
                row.get::<i64, &str>("room_id"),
                row.get::<String, &str>("name"),
                row.get::<bool, &str>("state"),
                row.get::<f32, &str>("power"),
                row.get::<f32, &str>("temperature"),
            );
            devices.push(dev);
        }
        Ok(devices)
    }

    pub async fn insert_device(
        &self,
        room_id: i64,
        name: String,
        dev: SmartDeviceType,
    ) -> Result<SmartDevice, sqlx::Error> {
        match dev {
            SmartDeviceType::Socket => {
                let socket = SmartSocket::new(&self.db, room_id, name).await?;
                Ok(SmartDevice::Socket(socket))
            }
            SmartDeviceType::Thermometer => {
                let thermo = SmartThermometer::new(&self.db, room_id, name).await?;
                Ok(SmartDevice::Thermometer(thermo))
            }
        }
    }

    pub async fn update_device(&self, id: i64, upd: SmartDeviceUpdate) -> Result<u64, sqlx::Error> {
        let dev_old = self.select_device_by_id(id).await?;
        match dev_old {
            SmartDevice::Socket(mut s) => s.update(&self.db, upd).await,
            SmartDevice::Thermometer(mut t) => t.update(&self.db, upd).await,
        }
    }

    pub async fn select_device_by_id(&self, id: i64) -> Result<SmartDevice, sqlx::Error> {
        let row = sqlx::query(format!("SELECT * FROM devices WHERE id = {};", id).as_str())
            .fetch_one(&self.db)
            .await?;
        let dev = SmartDevice::helper(
            SmartDeviceType::from_str(row.get::<&str, &str>("type")).unwrap(),
            row.get::<i64, &str>("id"),
            row.get::<i64, &str>("room_id"),
            row.get::<String, &str>("name"),
            row.get::<bool, &str>("state"),
            row.get::<f32, &str>("power"),
            row.get::<f32, &str>("temperature"),
        );
        Ok(dev)
    }

    pub async fn delete_device(&self, id: i64) -> Result<u64, sqlx::Error> {
        let sql = format!("DELETE FROM devices WHERE id = {};", id);
        let result = sqlx::query(sql.as_str()).execute(&self.db).await?;
        Ok(result.rows_affected())
    }
}
