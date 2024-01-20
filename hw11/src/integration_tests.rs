#![cfg(test)]

use hw11::smartdevices::socket::{SmartSocket, SmartSocketError};
use hw11::smartdevices::thermometer::SmartThermometer;
use hw11::smartdevices::{SmartDevice, SmartDeviceError};
use hw11::smarthouse::room::{SmartRoom, SmartRoomError};
use hw11::smarthouse::BorrowingDeviceInfoProvider;
use hw11::smarthouse::OwningDeviceInfoProvider;
use hw11::smarthouse::SmartHouse;

#[test]
fn smartsocket() {
    let mut ss = SmartSocket::new("name", "brief").unwrap();
    assert_eq!(ss.get_room().err(), Some(SmartDeviceError::NotInstalled));
    assert!(ss.set_room("room1").is_ok());
    assert!(ss.set_room("room2").is_err());
    assert_eq!(ss.get_room().ok(), Some(String::from("room1")));
    assert_ne!(ss.get_room().ok(), Some(String::from("room2")));

    ss.on();
    assert_ne!(ss.power_consumption(), 0.0);
    assert_ne!(ss.get_state(), "Socket off".to_string());

    ss.off();
    assert_eq!(ss.power_consumption(), 0.0);
    assert_eq!(ss.get_state(), "Socket off".to_string());
}

#[test]
fn smartthermometer() {
    let mut st = SmartThermometer::new("thermometer1", -40.0, 60.0).unwrap();
    assert_eq!(st.get_room().err(), Some(SmartDeviceError::NotInstalled));
    assert!(st.set_room("room1").is_ok());
    assert!(!st.set_room("room2").is_ok());
    assert_eq!(st.get_room().ok(), Some(String::from("room1")));
    assert_ne!(st.get_room().ok(), Some(String::from("room2")));
}

#[test]
fn smartroom() {
    let mut room = SmartRoom::new("room1").unwrap();
    let mut socket1 = SmartSocket::new("ssocket1", "brief1").unwrap();
    let mut socket2 = SmartSocket::new("ssocket1", "brief1").unwrap();
    assert!(room.add_device(&mut socket1).is_ok());
    assert!(!room.add_device(&mut socket2).is_ok());

    assert_eq!(room.devices.len(), 1);
}

#[test]
fn smartroom_negative() {
    let mut room1 = SmartRoom::new("room1").unwrap();
    let mut room2 = SmartRoom::new("room2").unwrap();
    let mut socket1 = SmartSocket::new("ssocket1", "brief1").unwrap();
    assert!(room1.add_device(&mut socket1).is_ok());
    let err1 = SmartSocketError::SetRoom(room1.name.clone());
    let err2 = SmartDeviceError::from(err1);
    let err3 = SmartRoomError::from(err2);
    assert_eq!(room2.add_device(&mut socket1).err(), Some(err3));
}

#[test]
fn smarthouse() {
    let mut house = SmartHouse::new("Sweet Home").unwrap();
    let mut room1 = SmartRoom::new("room1").unwrap();
    let mut room2 = SmartRoom::new("room2").unwrap();

    let mut socket1 = SmartSocket::new("socket1", "brief1").unwrap();
    let mut socket2 = SmartSocket::new("socket2", "brief2").unwrap();

    room1.add_device(&mut socket1).unwrap();
    room2.add_device(&mut socket2).unwrap();

    house.add_room(room1).unwrap();
    assert_eq!(house.get_rooms(), vec!["room1"]);

    house.add_room(room2).unwrap();
    let rooms = house.get_rooms();
    assert!(rooms.contains(&"room1".to_string()));
    assert!(rooms.contains(&"room2".to_string()));

    assert_eq!(house.devices("room1"), vec!["socket1"]);
    assert_eq!(house.devices("room2"), vec!["socket2"]);

    //Negative test. Room with name room1 is present in house
    let room1_1 = SmartRoom::new("room1").unwrap();
    assert!(house.add_room(room1_1).is_err());
}

#[test]
fn smarthouse_negative() {
    let mut house = SmartHouse::new("Sweet Home").unwrap();
    let room1 = SmartRoom::new("room1").unwrap();
    let room2 = SmartRoom::new("room1").unwrap();

    assert!(house.add_room(room1).is_ok());
    assert!(house.add_room(room2).is_err());
}

#[test]
fn device_info_provider() {
    let mut house = SmartHouse::new("Sweet Home").unwrap();
    let mut room1 = SmartRoom::new("room1").unwrap();
    let mut room2 = SmartRoom::new("room2").unwrap();

    let mut socket1 = SmartSocket::new("socket1", "brief1").unwrap();
    let mut socket2 = SmartSocket::new("socket2", "brief2").unwrap();

    room1.add_device(&mut socket1).unwrap();
    room2.add_device(&mut socket2).unwrap();

    house.add_room(room1).unwrap();
    house.add_room(room2).unwrap();

    let own_prov = OwningDeviceInfoProvider {
        devices: vec![Box::new(socket1)],
    };

    let report = house.create_report(&own_prov);
    assert_eq!(
        report,
        format!("Sweet Home\nRoom[room1] => device[socket1]:Socket off\n")
    );

    let borrow_prov = BorrowingDeviceInfoProvider {
        devices: vec![&socket2],
    };

    let report = house.create_report(&borrow_prov);

    assert_eq!(
        report,
        format!("Sweet Home\nRoom[room2] => device[socket2]:Socket off\n")
    );
}
