#![cfg(test)]
use hw6::{
    smartdevices::SmartDevice,
    smartdevices::{socket::SmartSocket, thermometer::SmartThermometer},
    smarthouse::room::SmartRoom,
    smarthouse::SmartHouse,
    smarthouse::{BorrowingDeviceInfoProvider, OwningDeviceInfoProvider},
};

#[test]
fn smartsocket() {
    let mut ss = SmartSocket::new("name", "brief");
    assert_eq!(ss.get_room(), "Not installed");
    assert!(ss.set_room("room1").is_ok());
    assert!(ss.set_room("room2").is_err());
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
    assert!(st.set_room("room1").is_ok());
    assert!(st.set_room("room2").is_err());
    assert_eq!(st.get_room(), "room1");
    assert_ne!(st.get_room(), "room2");
}

#[test]
fn smartroom() {
    let mut room = SmartRoom::new("room1");
    let mut socket1 = SmartSocket::new("ssocket1", "brief1");
    let mut socket2 = SmartSocket::new("ssocket1", "brief1");
    assert!(room.add_device(SmartDevice::from(&mut socket1)).is_ok());
    assert!(room.add_device(SmartDevice::from(&mut socket2)).is_err());

    assert_eq!(room.devices().len(), 1);

    assert!(room.del_device(String::from("ssocket1")).is_ok());
    assert!(room.del_device(String::from("ssocket2")).is_err());

    assert_eq!(room.devices().len(), 0);
}

#[test]
fn smarthouse() {
    let mut house = SmartHouse::new("Sweet Home");
    let mut room1 = SmartRoom::new("room1");
    let mut room2 = SmartRoom::new("room2");

    let mut socket1 = SmartSocket::new("socket1", "brief1");
    let mut socket2 = SmartSocket::new("socket2", "brief2");

    assert!(room1.add_device(SmartDevice::from(&mut socket1)).is_ok());
    assert!(room2.add_device(SmartDevice::from(&mut socket2)).is_ok());

    assert!(house.add_room(room1).is_ok());
    assert_eq!(house.get_rooms(), vec!["room1"]);

    assert!(house.add_room(room2).is_ok());
    let rooms = house.get_rooms();
    assert!(rooms.contains(&"room1".to_string()));
    assert!(rooms.contains(&"room2".to_string()));

    assert_eq!(house.devices("room1"), vec!["socket1"]);
    assert_eq!(house.devices("room2"), vec!["socket2"]);
}

#[test]
fn device_info_provider() {
    let mut house1 = SmartHouse::new("Sweet Home1");
    let mut house2 = SmartHouse::new("Sweet Home2");
    let mut room1 = SmartRoom::new("room1");
    let mut room2 = SmartRoom::new("room2");

    let mut socket1 = SmartSocket::new("socket1", "brief1");
    let mut socket2 = SmartSocket::new("socket2", "brief2");

    assert!(room1.add_device(SmartDevice::from(&mut socket1)).is_ok());
    assert!(room2.add_device(SmartDevice::from(&mut socket2)).is_ok());

    assert!(house1.add_room(room1).is_ok());

    let own_prov = OwningDeviceInfoProvider {
        devices: vec![SmartDevice::from(&mut socket1)],
    };

    let answer1 =
        String::from("Sweet Home1\nReport for room:room1\n   => Device[socket1]:Socket off\n\n");

    let report = house1.create_report(&own_prov);
    assert_eq!(report, answer1);

    assert!(house2.add_room(room2).is_ok());
    let borrow_sd = SmartDevice::from(&mut socket2);
    let borrow_prov = BorrowingDeviceInfoProvider {
        devices: vec![&borrow_sd],
    };

    let report = house2.create_report(&borrow_prov);

    let answer2 =
        String::from("Sweet Home2\nReport for room:room2\n   => Device[socket2]:Socket off\n\n");
    assert_eq!(report, answer2);
}

#[test]
fn negative_tests() {
    let mut room1 = SmartRoom::new("room1");

    let mut socket1 = SmartSocket::new("socket1", "brief1");
    let mut socket2 = SmartSocket::new("socket1", "brief2");

    let ret = room1.add_device(SmartDevice::from(&mut socket1));
    assert!(ret.is_ok());
    let ret = room1.add_device(SmartDevice::from(&mut socket2));
    assert!(ret.is_err());
}
