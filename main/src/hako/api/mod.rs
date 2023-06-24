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
    // pdu apis
    fn hako_asset_create_pdu_lchannel(robo_name: *const c_char, channel_id: i32, pdu_size: i32) -> bool;
    fn hako_asset_get_pdu_channel(robo_name: *const c_char, channel_id: i32) -> i32;
    fn hako_asset_is_pdu_dirty(asset_name: *const c_char, robo_name: *const c_char, channel_id: i32) -> bool;
    fn hako_asset_write_pdu(asset_name: *const c_char, robo_name: *const c_char, channel_id: i32, pdu_data: *const c_char, len: i32) -> bool;
    fn hako_asset_read_pdu(asset_name: *const c_char, robo_name: *const c_char, channel_id: i32, pdu_data: *mut u8, len: i32) -> bool;
    fn hako_asset_notify_read_pdu_done(asset_name: *const u8) -> bool;
    fn hako_asset_notify_write_pdu_done(asset_name: *const u8) -> bool;
    fn hako_asset_is_pdu_sync_mode(asset_name: *const u8) -> bool;
    fn hako_asset_is_simulation_mode() -> bool;
    fn hako_asset_is_pdu_created() -> bool;

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
            0 => SimulationStateType::Stopped,
            1 => SimulationStateType::Runnable,
            2 => SimulationStateType::Running,
            3 => SimulationStateType::Stopping,
            4 => SimulationStateType::Resetting,
            5 => SimulationStateType::Error,
            6 => SimulationStateType::Invalid,
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
// pdu apis
pub fn asset_create_pdu_lchannel(robo_name: String, channel_id: i32, pdu_size: i32) -> bool
{
    unsafe {
        let c_string: CString = CString::new(robo_name).unwrap();
        let c_string_ptr: *const c_char = c_string.as_ptr();
        hako_asset_create_pdu_lchannel(c_string_ptr, channel_id, pdu_size)
    }
}
pub fn asset_get_pdu_channel(robo_name: String, channel_id: i32) -> i32
{
    unsafe {
        let c_string: CString = CString::new(robo_name).unwrap();
        let c_string_ptr: *const c_char = c_string.as_ptr();
        hako_asset_get_pdu_channel(c_string_ptr, channel_id)
    }
}
pub fn asset_is_pdu_dirty(asset_name: String, robo_name: String, channel_id: i32) -> bool
{
    unsafe {
        let asset_c_string: CString = CString::new(asset_name).unwrap();
        let asset_c_string_ptr: *const c_char = asset_c_string.as_ptr();
        let robo_c_string: CString = CString::new(robo_name).unwrap();
        let robo_c_string_ptr: *const c_char = robo_c_string.as_ptr();
        hako_asset_is_pdu_dirty(asset_c_string_ptr, robo_c_string_ptr, channel_id)
    }
}

pub fn asset_write_pdu(asset_name: String, robo_name: String, channel_id: i32, pdu_data: *const c_char, len: i32) -> bool
{
    unsafe {
        let asset_c_string: CString = CString::new(asset_name).unwrap();
        let asset_c_string_ptr: *const c_char = asset_c_string.as_ptr();
        let robo_c_string: CString = CString::new(robo_name).unwrap();
        let robo_c_string_ptr: *const c_char = robo_c_string.as_ptr();
        hako_asset_write_pdu(asset_c_string_ptr, robo_c_string_ptr, channel_id, pdu_data, len)
    }
}

pub fn asset_read_pdu(asset_name: String, robo_name: String, channel_id: i32, pdu_data: *mut u8, len: i32) -> bool
{
    unsafe {
        let asset_c_string: CString = CString::new(asset_name).unwrap();
        let asset_c_string_ptr: *const c_char = asset_c_string.as_ptr();
        let robo_c_string: CString = CString::new(robo_name).unwrap();
        let robo_c_string_ptr: *const c_char = robo_c_string.as_ptr();
        hako_asset_read_pdu(asset_c_string_ptr, robo_c_string_ptr, channel_id, pdu_data, len)
    }
}

pub fn asset_notify_read_pdu_done(asset_name: String) -> bool
{
    unsafe {
        let asset_c_string: CString = CString::new(asset_name).unwrap();
        let asset_c_string_ptr: *const c_char = asset_c_string.as_ptr();
        hako_asset_notify_read_pdu_done(asset_c_string_ptr)
    }
}

pub fn asset_notify_write_pdu_done(asset_name: String) -> bool
{
    unsafe {
        let asset_c_string: CString = CString::new(asset_name).unwrap();
        let asset_c_string_ptr: *const c_char = asset_c_string.as_ptr();
        hako_asset_notify_write_pdu_done(asset_c_string_ptr)        
    }
}

pub fn asset_is_pdu_sync_mode(asset_name: String) -> bool
{
    unsafe {
        let asset_c_string: CString = CString::new(asset_name).unwrap();
        let asset_c_string_ptr: *const c_char = asset_c_string.as_ptr();
        hako_asset_is_pdu_sync_mode(asset_c_string_ptr)        
    }
}

pub fn asset_is_simulation_mode() -> bool
{
    unsafe {
        hako_asset_is_simulation_mode()        
    }
}

pub fn asset_is_pdu_created() -> bool
{
    unsafe {
        hako_asset_is_pdu_created()        
    }
}
