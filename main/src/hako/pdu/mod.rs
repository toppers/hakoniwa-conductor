extern crate lazy_static;
extern crate once_cell;
use std::{sync::Mutex, collections::HashMap};
use once_cell::sync::Lazy;
use crate::hako::api;
use libc::c_char;

pub const ASSET_PACKET_MAX_SIZE: usize = 4096;


pub struct AssetPubPduType {
    pub asset_name: String,
    pub robo_name: String,
    pub pdu_size: i32,
    pub method_type: String,
    pub channel_id: i32
}
pub struct AsssetSubPduOptionType {
    pub udp_ip_port: String
}
pub struct AssetSubPduType {
    pub asset_name: String,
    pub robo_name: String,
    pub pdu_size: i32,
    pub buffer: [u8; ASSET_PACKET_MAX_SIZE],
    pub method_type: String,
    pub options: AsssetSubPduOptionType,
    pub channel_id: i32
}
pub static ASSET_SUB_PDU_CHANNELS: Lazy<Mutex<HashMap<i32, AssetSubPduType>>> = Lazy::new(|| {
    let m = HashMap::new();
    Mutex::new(m)
});
pub static ASSET_PUB_PDU_CHANNELS: Lazy<Mutex<HashMap<i32, AssetPubPduType>>> = Lazy::new(|| {
    let m = HashMap::new();
    Mutex::new(m)
});


pub fn get_subscribers(v: &mut Vec<i32>)
{
    let map = ASSET_SUB_PDU_CHANNELS.lock().unwrap();
    for (key, _value) in map.iter() {
        v.push(key.clone());
    }
}

pub fn create_asset_sub_pdu(asset_name: String, robo_name: String, channel_id: i32, pdu_size: i32, udp_ip_port: String, method_type: String) -> bool
{
    println!("create_asset_sub_pdu");
    let real_id = api::asset_get_pdu_channel(robo_name.clone(), channel_id);
    let mut map = ASSET_SUB_PDU_CHANNELS.lock().unwrap();
    match map.get(&real_id) {
        Some(_n) => {
            return false;
        },
        None => {
            let pdu = AssetSubPduType {
                asset_name: asset_name.clone(),
                robo_name: robo_name.clone(),
                options: { 
                    AsssetSubPduOptionType {
                        udp_ip_port: udp_ip_port,
                    }
                },
                pdu_size: pdu_size,
                buffer: [0; ASSET_PACKET_MAX_SIZE ],
                method_type: method_type,
                channel_id: channel_id
            };
            map.insert(real_id, pdu);
            return true;
        }
    };
}

pub fn remove_asset_pub_pdu(robo_name: String, channel_id: i32)
{
    println!("remove_asset_pub_pdu");
    let real_id = api::asset_get_pdu_channel(robo_name, channel_id);
    let mut map = ASSET_PUB_PDU_CHANNELS.lock().unwrap();
    map.remove(&real_id);
}

pub fn remove_asset_sub_pdu(robo_name: String, channel_id: i32)
{
    println!("remove_asset_sub_pdu");
    let real_id = api::asset_get_pdu_channel(robo_name, channel_id);
    let mut map = ASSET_SUB_PDU_CHANNELS.lock().unwrap();
    map.remove(&real_id);
}

pub fn get_asset_sub_pdu_size(robo_name: String, channel_id: i32) -> i32
{
    println!("get_asset_sub_pdu_size");
    let real_id = api::asset_get_pdu_channel(robo_name, channel_id);
    let map = ASSET_SUB_PDU_CHANNELS.lock().unwrap();
    let size = match map.get(&real_id) {
        Some(_n) => _n.pdu_size,
        None => -1
    };
    size
}
pub fn get_asset_pub_pdu_channel_robo_name(real_id: i32) -> (i32, String)
{
    let map = ASSET_PUB_PDU_CHANNELS.lock().unwrap();
    let ret = match map.get(&real_id) {
        Some(_n) => (_n.channel_id, _n.robo_name.clone()),
        None => (-1, String::new())
    };
    ret
}

pub fn create_asset_pub_pdu(asset_name: String, robo_name: String, channel_id: i32, pdu_size: i32, method_type: String) -> bool
{
    let real_id = api::asset_get_pdu_channel(robo_name.clone(), channel_id);
    println!("create_asset_pub_pdu: robo_name={} channel_id={} real_id={}", robo_name.clone(), channel_id, real_id);
    let mut map = ASSET_PUB_PDU_CHANNELS.lock().unwrap();
    match map.get(&real_id) {
        Some(_n) => {
            return false;
        },
        None => {
            let pdu = AssetPubPduType {
                asset_name: asset_name.clone(),
                robo_name: robo_name.clone(),
                pdu_size: pdu_size,
                method_type: method_type,
                channel_id: channel_id
            };
            println!("create_asset_pub_pdu: channel_ID={}", channel_id);
            map.insert(real_id, pdu);
            return true;
        }
    };
}

pub fn write_asset_pub_pdu(robo_name: String, channel_id: i32, data: &[u8], size: usize) -> bool
{
    let real_id = api::asset_get_pdu_channel(robo_name.clone(), channel_id);
    //println!("write_asset_pub_pdu: robo_name={} channel_id={} real_id={}", robo_name.clone(), channel_id, real_id);
    let map = ASSET_PUB_PDU_CHANNELS.lock().unwrap();
    let pdu = map.get(&real_id).unwrap();
    let ret = api::asset_write_pdu(
        pdu.asset_name.clone(), 
        pdu.robo_name.clone(), 
        channel_id.clone(), 
        data.as_ptr() as *const c_char, 
        size as i32);
    //if ret {
    //    api::asset_notify_write_pdu_done(pdu.asset_name.clone());
    //}
    //println!("asset_notify_write_pdu_done: channel_id={:?} ret={:?}", channel_id, ret);
    ret
}
