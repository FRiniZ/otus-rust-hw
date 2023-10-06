#![cfg(test)]
use crate::smartdevices::socket::SmartSocket;
use crate::smartdevices::thermometer::SmartThermometer;
use crate::smartdevices::SmartDevice;
use crate::smarthouse::room::SmartRoom;
use crate::smarthouse::BorrowingDeviceInfoProvider;
use crate::smarthouse::OwningDeviceInfoProvider;
use crate::smarthouse::SmartHouse;

#[test]
fn smartsocket() {
    let mut ss = SmartSocket::new("name", "brief");
    assert_eq!(ss.get_room(), "Not installed");
    assert!(ss.set_room("room1"));
    assert!(!ss.set_room("room2"));
    assert_eq!(ss.get_room(), "room1");
    assert_ne!(ss.get_room(), "room2");

    ss.on();
    assert_ne!(ss.power_consumption(), 0.0);
    assert_ne!(ss.get_state(), "Socket off".to_string());

    ss.off();
    assert_eq!(ss.power_consumption(), 0.0);
    assert_eq!(ss.get_state(), "Socket off".to_string());
}

#[test]
fn smartthermometer() {
    let mut st = SmartThermometer::new("thermometer1", -40.0, 60.0);
    assert_eq!(st.get_room(), "Not installed");
    assert!(st.set_room("room1"));
    assert!(!st.set_room("room2"));
    assert_eq!(st.get_room(), "room1");
    assert_ne!(st.get_room(), "room2");
}

#[test]
fn smartroom() {
    let mut room = SmartRoom::new("room1");
    let mut socket1 = SmartSocket::new("ssocket1", "brief1");
    let mut socket2 = SmartSocket::new("ssocket1", "brief1");
    assert!(room.add_device(&mut socket1));
    assert!(!room.add_device(&mut socket2));

    assert_eq!(room.devices.len(), 1);
}

#[test]
fn smarthouse() {
    let mut house = SmartHouse::new("Sweet Home");
    let mut room1 = SmartRoom::new("room1");
    let mut room2 = SmartRoom::new("room2");

    let mut socket1 = SmartSocket::new("socket1", "brief1");
    let mut socket2 = SmartSocket::new("socket2", "brief2");

    room1.add_device(&mut socket1);
    room2.add_device(&mut socket2);

    house.add_room(room1);
    assert_eq!(house.get_rooms(), vec!["room1"]);

    house.add_room(room2);
    let rooms = house.get_rooms();
    assert!(rooms.contains(&"room1".to_string()));
    assert!(rooms.contains(&"room2".to_string()));

    assert_eq!(house.devices("room1"), vec!["socket1"]);
    assert_eq!(house.devices("room2"), vec!["socket2"]);
}

#[test]
fn device_info_provider() {
    let mut house = SmartHouse::new("Sweet Home");
    let mut room1 = SmartRoom::new("room1");
    let mut room2 = SmartRoom::new("room2");

    let mut socket1 = SmartSocket::new("socket1", "brief1");
    let mut socket2 = SmartSocket::new("socket2", "brief2");

    room1.add_device(&mut socket1);
    room2.add_device(&mut socket2);

    house.add_room(room1);
    house.add_room(room2);

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
