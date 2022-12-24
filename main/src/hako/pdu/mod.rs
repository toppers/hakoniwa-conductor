extern crate lazy_static;
extern crate once_cell;
use std::{sync::Mutex, collections::HashMap};
use once_cell::sync::Lazy;

static mut PDU_SERVER_PORT: i32 = -1;
const ASSET_SUB_PDU_MAX_SIZE: usize = 4096;
struct AssetPubPduType {
    pdu_size: i32,
    buffer: [u8; ASSET_SUB_PDU_MAX_SIZE]
}
struct AssetSubPduType {
    pdu_size: i32,
    udp_port: i32
}
static ASSET_SUB_PDU_CHANNELS: Lazy<Mutex<HashMap<i32, AssetSubPduType>>> = Lazy::new(|| {
    let m = HashMap::new();
    Mutex::new(m)
});
static ASSET_PUB_PDU_CHANNELS: Lazy<Mutex<HashMap<i32, AssetPubPduType>>> = Lazy::new(|| {
    let m = HashMap::new();
    Mutex::new(m)
});

pub fn set_server_port(port: i32)
{
    unsafe {
        PDU_SERVER_PORT = port;
    }
}
pub fn get_server_port() -> i32
{
    unsafe {
        return PDU_SERVER_PORT;
    }
}

pub fn create_asset_sub_pdu(channel_id: i32, pdu_size: i32, udp_port: i32) -> bool
{
    let mut map = ASSET_SUB_PDU_CHANNELS.lock().unwrap();
    match map.get(&channel_id) {
        Some(_n) => {
            return false;
        },
        None => {
            let pdu = AssetSubPduType {
                udp_port: udp_port,
                pdu_size: pdu_size    
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
pub fn get_asset_sub_pdu_port(channel_id: i32) -> i32
{
    let map = ASSET_SUB_PDU_CHANNELS.lock().unwrap();
    let port = match map.get(&channel_id) {
        Some(_n) => _n.udp_port,
        None => -1
    };
    port
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


pub fn create_asset_pub_pdu(channel_id: i32, pdu_size: i32) -> bool
{
    let mut map = ASSET_PUB_PDU_CHANNELS.lock().unwrap();
    match map.get(&channel_id) {
        Some(_n) => {
            return false;
        },
        None => {
            let pdu = AssetPubPduType {
                pdu_size: pdu_size,
                buffer: [0; ASSET_SUB_PDU_MAX_SIZE ]
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
pub fn write_asset_pub_pdu(channel_id: i32, data: *const u8, size: usize)
{
    let map = ASSET_PUB_PDU_CHANNELS.lock().unwrap();
    let mut buffer = map.get(&channel_id).unwrap().buffer;
    let mut i: usize = 0;
    while i < size {
        unsafe {
            buffer[i] = *data.offset(i as isize) as u8;
        }
        i = i + 1;
    }
}
