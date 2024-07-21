use std::ffi::{c_int, c_uint, c_void};

extern "C" {
    pub fn get_global_variable(pos: c_uint) -> *mut c_void;
    pub fn set_global_variable(data: *mut c_void, pos: c_uint) -> c_int;
    pub fn clear_variable(pos: c_uint) -> c_int;
    pub fn has_data_at(pos: c_uint) -> c_int;
    pub fn is_null_ptr(ptr: *mut c_void) -> c_int;
}
