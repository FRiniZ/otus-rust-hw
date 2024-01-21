#[cfg(test)]
use super::*;

#[test]
fn test_new() {
    let st = SmartThermometer::new("thermometer1", -40.0, 60.0).unwrap();
    assert_eq!(st.name, "thermometer1");
    assert_eq!(st._level_min, -40.0);
    assert_eq!(st._level_max, 60.0);
    assert_eq!(st.room, None)
}
