#[cfg(test)]
use super::*;

#[test]
fn test_new() {
    let ss = SmartSocket::new("name", "brief");
    assert_eq!(ss.name, "name");
    assert_eq!(ss.brief, "brief");
}
