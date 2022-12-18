extern crate link_cplusplus;

use libc::c_char;
use std::ffi::CString;

#[link(name = "shakoc")]
extern "C" {
    /*
     * Master API
     */
    fn hako_master_init() -> bool;
    fn hako_master_execute() -> bool;
    fn hako_master_set_config_simtime(max_delay_time_usec: i64, delta_time_usec: i64);

    /*
     * Asset API
     */
    fn hako_asset_init() -> bool;
    fn hako_asset_register_polling(name: *const c_char) -> bool;
    fn hako_asset_unregister(name: *const c_char) -> bool;
    fn hako_asset_notify_simtime(name: *const c_char, simtime: i64);
    fn hako_asset_get_worldtime() -> i64;
    fn hako_asset_get_event(name: *const c_char) -> i32;
    fn hako_asset_start_feedback(name: *const c_char, is_ok: bool) -> bool;
    fn hako_asset_stop_feedback(name: *const c_char, is_ok: bool) -> bool;
    fn hako_asset_reset_feedback(name: *const c_char, is_ok: bool) -> bool;
    /*
     * Simulation API
     */
    fn hako_simevent_get_state() -> i32;
    fn hako_simevent_start() -> bool;
    fn hako_simevent_stop() -> bool;
    fn hako_simevent_reset() -> bool;
}

#[derive(Debug)]
pub enum SimulationAssetEventType
{
    None,
    Start,
    Stop,
    Reset,
    Error,
    Invalid
}
#[derive(Debug)]
pub enum SimulationStateType
{
    None,
    Stopped,
    Runnable,
    Running,
    Stopping,
    Resetting,
    Error,
    Invalid
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


pub fn asset_unregister(name: String) -> bool
{
    let c_string: CString = CString::new(name).unwrap();
    let c_string_ptr: *const c_char = c_string.as_ptr();
    unsafe {
        hako_asset_unregister(c_string_ptr)
    }
}

pub fn asset_notify_simtime(name: String, simtime: i64)
{
    let c_string: CString = CString::new(name).unwrap();
    let c_string_ptr: *const c_char = c_string.as_ptr();
    unsafe {
        hako_asset_notify_simtime(c_string_ptr, simtime)
    }
}
pub fn asset_get_worldtime() -> i64
{
    unsafe {
        hako_asset_get_worldtime()
    }
}
pub fn asset_get_event(name: String) -> SimulationAssetEventType
{
    let c_string: CString = CString::new(name).unwrap();
    let c_string_ptr: *const c_char = c_string.as_ptr();
    unsafe {
        let ev: i32 = hako_asset_get_event(c_string_ptr);
        match ev {
            0 => SimulationAssetEventType::None,
            1 => SimulationAssetEventType::Start,
            2 => SimulationAssetEventType::Stop,
            3 => SimulationAssetEventType::Reset,
            4 => SimulationAssetEventType::Error,
            _ => SimulationAssetEventType::Invalid,
        }
    }
}
pub fn asset_start_feedback(name: String, is_ok: bool) -> bool
{
    let c_string: CString = CString::new(name).unwrap();
    let c_string_ptr: *const c_char = c_string.as_ptr();
    unsafe {
        hako_asset_start_feedback(c_string_ptr, is_ok)
    }
}
pub fn asset_stop_feedback(name: String, is_ok: bool) -> bool
{
    let c_string: CString = CString::new(name).unwrap();
    let c_string_ptr: *const c_char = c_string.as_ptr();
    unsafe {
        hako_asset_stop_feedback(c_string_ptr, is_ok)
    }
}
pub fn asset_reset_feedback(name: String, is_ok: bool) -> bool
{
    let c_string: CString = CString::new(name).unwrap();
    let c_string_ptr: *const c_char = c_string.as_ptr();
    unsafe {
        hako_asset_reset_feedback(c_string_ptr, is_ok)
    }
}
pub fn simevent_get_state() -> SimulationStateType
{
    unsafe {
        let ev = hako_simevent_get_state();
        match ev {
            0 => SimulationStateType::None,
            1 => SimulationStateType::Stopped,
            2 => SimulationStateType::Runnable,
            3 => SimulationStateType::Running,
            4 => SimulationStateType::Stopping,
            5 => SimulationStateType::Resetting,
            6 => SimulationStateType::Error,
            _ => SimulationStateType::Invalid,
        }
    }
}
pub fn simevent_start() -> bool
{
    unsafe {
        hako_simevent_start()
    }
}
pub fn simevent_stop() -> bool
{
    unsafe {
        hako_simevent_stop()
    }
}
pub fn simevent_reset() -> bool
{
    unsafe {
        hako_simevent_reset()
    }
}
