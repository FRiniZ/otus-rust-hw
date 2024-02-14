use std::{ffi::CString, os::raw::c_char};

use rand::Rng;

#[derive(Default)]
pub struct SmartSocket {
    pub state: bool,
    pub power: f32,
    _msg: CString,
}

#[no_mangle]
pub extern "C" fn smart_socket_new() -> *mut SmartSocket {
    let ss = SmartSocket::default();
    Box::into_raw(Box::new(ss))
}

#[no_mangle]
pub extern "C" fn smart_socket_on(ss: &mut SmartSocket) -> i32 {
    if !ss.state {
        ss.state = true;
        0
    } else {
        1
    }
}

#[no_mangle]
pub extern "C" fn smart_socket_off(ss: &mut SmartSocket) -> i32 {
    if ss.state {
        ss.state = false;
        0
    } else {
        1
    }
}

#[no_mangle]
pub extern "C" fn smart_socket_state(ss: &mut SmartSocket) -> *const c_char {
    if ss.state {
        ss._msg = CString::new(format!(
            "On:Power {}",
            rand::thread_rng().gen_range(190.00..230.00)
        ))
        .unwrap();
    } else {
        ss._msg = CString::new("Off:Power 0.0").unwrap();
    }
    ss._msg.as_ptr()
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
