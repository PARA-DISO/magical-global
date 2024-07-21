use magical_global_sys::*;
use std::boxed::Box;
use std::ffi::{c_int, c_uint, c_void};

#[derive(PartialEq)]
pub enum ReturnCode {
    Success,
    UndefinedError,
    OutOfRange = -1,
    NotAvailable = -2,
}
impl ReturnCode {
    pub fn is_success(&self) -> bool {
        self == &ReturnCode::Success
    }
    pub fn into_msg(self) -> String {
        match self {
            Self::Success => "Success".to_string(),
            Self::UndefinedError => "Undefined Error".to_string(),
            Self::OutOfRange => "Out of Range".to_string(),
            Self::NotAvailable => "Not Available".to_string(),
        }
    }
}
impl From<c_int> for ReturnCode {
    fn from(code: c_int) -> Self {
        match code {
            0 => ReturnCode::Success,
            -1 => ReturnCode::OutOfRange,
            -2 => ReturnCode::NotAvailable,
            _ => ReturnCode::UndefinedError,
        }
    }
}

pub fn set_at<T>(data: Box<T>, pos: u32) -> Result<(), (String, Box<T>)> {
    let ptr = Box::into_raw(data);
    let res_code =
        unsafe { magical_global_sys::set_global_variable(ptr as *mut c_void, pos as c_uint) };
    let res_code = ReturnCode::from(res_code);
    if res_code.is_success() {
        return Ok(());
    } else {
        return Err((res_code.into_msg(), unsafe { Box::<T>::from_raw(ptr) }));
    }
}
pub fn get_at_mut<T>(pos: u32) -> Option<&'static mut T> {
    unsafe {
        let ptr = magical_global_sys::get_global_variable(pos as c_uint) as *mut T;
        ptr.as_mut()
    }
}
pub fn get_at<T>(pos: u32) -> Option<&'static T> {
    unsafe {
        let ptr = magical_global_sys::get_global_variable(pos as c_uint) as *mut T;
        ptr.as_ref()
    }
}
pub fn is_empty_at(pos: u32) -> bool {
    let has_data = unsafe { magical_global_sys::has_data_at(pos as c_uint) };
    has_data == 0
}
pub fn take_at<T>(pos: u32) -> Option<T> {
    unsafe {
        let ptr = magical_global_sys::get_global_variable(pos as c_uint);
        if is_null_ptr(ptr) == 0 {
            let _ = magical_global_sys::clear_variable(pos as c_uint);
            Some((ptr as *mut T).read())
        } else {
            None
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_exist() {
        assert_eq!(is_empty_at(0), true);
        set_at(std::boxed::Box::new(0u32), 0);
        assert_eq!(is_empty_at(0), false);
        let _ = take_at::<u32>(0).unwrap();
        assert_eq!(is_empty_at(0), true);
    }
    #[test]
    fn set_and_get() {
        let e_code = set_at(std::boxed::Box::new(255u32), 0);
        assert_eq!(e_code, Ok(()));
        let ref_data = get_at_mut::<u32>(0);
        assert_eq!(ref_data, Some(&mut 255));
        let ref_data = get_at::<u32>(0);
        assert_eq!(ref_data, Some(&255));
        let data = take_at::<u32>(0);
        assert_eq!(data, Some(255));
    }
}
