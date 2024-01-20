#[cfg(test)]
use super::*;

#[test]
fn test_new() {
    let room = SmartRoom::new("room1").unwrap();
    assert_eq!(room.name, "room1");
}
