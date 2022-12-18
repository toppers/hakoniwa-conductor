extern crate link_cplusplus;

use libc::c_char;
use std::ffi::CString;

#[link(name = "shakoc")]
extern "C" {
    fn hako_master_init() -> bool;
    fn hako_master_execute() -> bool;
    fn hako_master_set_config_simtime(max_delay_time_usec: i64, delta_time_usec: i64);

    fn hako_asset_init() -> bool;
    fn hako_asset_register_polling(name: *const c_char) -> bool;
}

pub fn master_init(max_delay_time_usec: i64, delta_time_usec: i64)
{
    unsafe {
        hako_master_init();
        hako_master_set_config_simtime(max_delay_time_usec, delta_time_usec);
    }
}

pub fn master_execute() -> bool
{
    unsafe {
        hako_master_execute()
    }
}

pub fn asset_init() -> bool
{
    unsafe {
        hako_asset_init()
    }
}

pub fn asset_register_polling(name: String) -> bool
{
    let c_string: CString = CString::new(name).unwrap();
    let c_string_ptr: *const c_char = c_string.as_ptr();
    unsafe {
        hako_asset_register_polling(c_string_ptr)
    }
}