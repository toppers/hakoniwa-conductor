extern crate lazy_static;
extern crate once_cell;
use std::{sync::Mutex, collections::HashMap};
use once_cell::sync::Lazy;
use std::net::UdpSocket;
use std::str;

const ASSET_PACKET_MAX_SIZE: usize = 4096;
struct AssetPubPduType {
    asset_name: String,
    pdu_size: i32,
    buffer: [u8; ASSET_PACKET_MAX_SIZE]
}
struct AssetSubPduType {
    asset_name: String,
    pdu_size: i32,
    buffer: [u8; ASSET_PACKET_MAX_SIZE],
    udp_ip_port: String
}
static ASSET_SUB_PDU_CHANNELS: Lazy<Mutex<HashMap<i32, AssetSubPduType>>> = Lazy::new(|| {
    let m = HashMap::new();
    Mutex::new(m)
});
static ASSET_PUB_PDU_CHANNELS: Lazy<Mutex<HashMap<i32, AssetPubPduType>>> = Lazy::new(|| {
    let m = HashMap::new();
    Mutex::new(m)
});

static mut PDU_SERVER_PORT: i32 = -1;

pub fn activate_server(ip_port: &String)
{
    let v: Vec<&str> = ip_port.split(':').collect();
    unsafe {
        PDU_SERVER_PORT = String::from(v[0]).parse::<i32>().unwrap();
    }
    let socket = UdpSocket::bind(ip_port).unwrap();
    std::thread::spawn(move || {
        let mut buf = [0; ASSET_PACKET_MAX_SIZE];
        loop {
            match socket.recv_from(&mut buf) {
                Ok((_buf_size, _src_addr)) => {
                  //0..3: channel id
                  //4..7: bufsize
                  let mut buf_ch = [0;4];
                  let mut buf_sz = [0;4];
                  for i in 0..4 {
                    buf_ch[i] = buf[i];
                    buf_sz[i] = buf[i + 4];
                  }
                  let channel_id = i32::from_le_bytes(buf_ch);
                  let pdu_size = i32::from_le_bytes(buf_sz);
                  //8..bufsize: buffer
                  write_asset_pub_pdu(channel_id, &buf[8..], pdu_size as usize);
                },
                Err(e) => {
                  println!("couldn't recieve request: {:?}", e);
                }
              }
        }
    });
}
pub fn get_server_port() -> i32
{
    unsafe {
        return PDU_SERVER_PORT;
    }
}

pub fn get_subscribers(v: &mut Vec<i32>)
{
    let map = ASSET_SUB_PDU_CHANNELS.lock().unwrap();
    for (key, _value) in map.iter() {
        v.push(key.clone());
    }
}

pub fn create_publisher_udp_socket(udp_ip_port: &String) -> UdpSocket
{
    let socket = UdpSocket::bind(udp_ip_port).unwrap();
    socket
}


pub fn send_to_subscriber(socket: UdpSocket, channel_id: i32, data: &[u8], size: usize)
{
    let mut map = ASSET_SUB_PDU_CHANNELS.lock().unwrap();
    let pdu: &mut AssetSubPduType = map.get_mut(&channel_id).unwrap();
    //0..3: channel id
    //4..7: bufsize
    let buf_ch = i32::to_le_bytes(channel_id);
    let buf_sz = i32::to_le_bytes(size as i32);

    let buf = pdu.buffer.as_mut_slice();
    for i in 0..4 {
        buf[i] = buf_ch[i as usize];
        buf[i + 4] = buf_sz[i as usize];
    }

    socket.send_to(&pdu.buffer, pdu.udp_ip_port.clone()).expect("couldn't send data");
}

pub fn create_asset_sub_pdu(asset_name: String, channel_id: i32, pdu_size: i32, udp_ip_port: String) -> bool
{
    let mut map = ASSET_SUB_PDU_CHANNELS.lock().unwrap();
    match map.get(&channel_id) {
        Some(_n) => {
            return false;
        },
        None => {
            let pdu = AssetSubPduType {
                asset_name: asset_name,
                udp_ip_port: udp_ip_port,
                pdu_size: pdu_size,
                buffer: [0; ASSET_PACKET_MAX_SIZE ]
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


pub fn create_asset_pub_pdu(asset_name: String, channel_id: i32, pdu_size: i32) -> bool
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
                buffer: [0; ASSET_PACKET_MAX_SIZE ]
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
pub fn write_asset_pub_pdu(channel_id: i32, data: &[u8], size: usize)
{
    let map = ASSET_PUB_PDU_CHANNELS.lock().unwrap();
    let mut buffer = map.get(&channel_id).unwrap().buffer;
    let mut i: usize = 0;
    while i < size {
        buffer[i] = data[i];
        i = i + 1;
    }
}
