extern crate lazy_static;
extern crate once_cell;
use std::{sync::Mutex, collections::HashMap};
use once_cell::sync::Lazy;
use crate::hako::api;
use libc::c_char;

pub const ASSET_PACKET_MAX_SIZE: usize = 4096;


pub struct AssetPubPduType {
    pub asset_name: String,
    pub pdu_size: i32,
    pub method_type: String
}
pub struct AsssetSubPduOptionType {
    pub udp_ip_port: String
}
pub struct AssetSubPduType {
    pub asset_name: String,
    pub pdu_size: i32,
    pub buffer: [u8; ASSET_PACKET_MAX_SIZE],
    pub method_type: String,
    pub options: AsssetSubPduOptionType
}
pub static ASSET_SUB_PDU_CHANNELS: Lazy<Mutex<HashMap<i32, AssetSubPduType>>> = Lazy::new(|| {
    let m = HashMap::new();
    Mutex::new(m)
});
static ASSET_PUB_PDU_CHANNELS: Lazy<Mutex<HashMap<i32, AssetPubPduType>>> = Lazy::new(|| {
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

pub fn create_asset_sub_pdu(asset_name: String, channel_id: i32, pdu_size: i32, udp_ip_port: String, method_type: String) -> bool
{
    let mut map = ASSET_SUB_PDU_CHANNELS.lock().unwrap();
    match map.get(&channel_id) {
        Some(_n) => {
            return false;
        },
        None => {
            let pdu = AssetSubPduType {
                asset_name: asset_name,
                options: { 
                    AsssetSubPduOptionType {
                        udp_ip_port: udp_ip_port,
                    }
                },
                pdu_size: pdu_size,
                buffer: [0; ASSET_PACKET_MAX_SIZE ],
                method_type: method_type
            };
            map.insert(channel_id, pdu);
            return true;
        }
    };
}

pub fn remove_asset_sub_pdu(channel_id: i32)
{
    let mut map = ASSET_SUB_PDU_CHANNELS.lock().unwrap();
    map.remove(&channel_id);
}

pub fn get_asset_sub_pdu_size(channel_id: i32) -> i32
{
    let map = ASSET_SUB_PDU_CHANNELS.lock().unwrap();
    let size = match map.get(&channel_id) {
        Some(_n) => _n.pdu_size,
        None => -1
    };
    size
}


pub fn create_asset_pub_pdu(asset_name: String, channel_id: i32, pdu_size: i32, method_type: String) -> bool
{
    let mut map = ASSET_PUB_PDU_CHANNELS.lock().unwrap();
    match map.get(&channel_id) {
        Some(_n) => {
            return false;
        },
        None => {
            let pdu = AssetPubPduType {
                asset_name: asset_name,
                pdu_size: pdu_size,
                method_type: method_type
            };
            map.insert(channel_id, pdu);
            return true;
        }
    };
}

pub fn remove_asset_pub_pdu(channel_id: i32)
{
    let mut map = ASSET_PUB_PDU_CHANNELS.lock().unwrap();
    map.remove(&channel_id);
}

pub fn get_asset_pub_pdu_size(channel_id: i32) -> i32
{
    let map = ASSET_PUB_PDU_CHANNELS.lock().unwrap();
    let size = match map.get(&channel_id) {
        Some(_n) => _n.pdu_size,
        None => -1
    };
    size
}
pub fn write_asset_pub_pdu(channel_id: i32, data: &[u8], size: usize) -> bool
{
    let map = ASSET_PUB_PDU_CHANNELS.lock().unwrap();
    let pdu = map.get(&channel_id).unwrap();
    api::asset_write_pdu(
        pdu.asset_name.as_ptr() as *const c_char, 
        channel_id.clone(), 
        data.as_ptr() as *const c_char, 
        size as i32)
}
