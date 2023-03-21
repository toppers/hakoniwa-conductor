
extern crate lazy_static;
extern crate once_cell;
use std::net::UdpSocket;
use std::str;
use crate::hako::api;
use libc::c_char;

static mut PDU_SERVER_PORT: i32 = -1;
const ASSET_RECV_PACKET_MAX_SIZE: usize = 1024 * 1024;
use crate::hako::pdu::ASSET_PACKET_MAX_SIZE;
use crate::hako::pdu::ASSET_SUB_PDU_CHANNELS;
use crate::hako::pdu::AssetSubPduType;
use crate::hako::pdu::write_asset_pub_pdu;

pub fn activate_server(ip_port: &String)
{
    let v: Vec<&str> = ip_port.split(':').collect();
    println!("OPEN RECIEVER UDP PORT={}", ip_port);
    unsafe {
        PDU_SERVER_PORT = String::from(v[1]).parse::<i32>().unwrap();
    }
    let socket = UdpSocket::bind(ip_port).unwrap();
    std::thread::spawn(move || {
        let mut buf : Box<[u8]> = Box::new([0; ASSET_RECV_PACKET_MAX_SIZE]);
        loop {
            match socket.recv_from(&mut buf) {
                Ok((_buf_size, _src_addr)) => {
                  if _buf_size > ASSET_RECV_PACKET_MAX_SIZE {
                    println!("UDP recv buffer size(={}) is over max size(={})\n", _buf_size, ASSET_RECV_PACKET_MAX_SIZE);
                  }
                  else {
                    //0..3: channel id
                    //4..7: bufsize
                    //8..12: namelen
                    let mut buf_ch = [0;4];
                    let mut buf_sz = [0;4];
                    let mut buf_nl = [0;4];
                    for i in 0..4 {
                        buf_ch[i] = buf[i];
                        buf_sz[i] = buf[i + 4];
                        buf_nl[i] = buf[i + 8];
                    }
                    let channel_id = i32::from_le_bytes(buf_ch);
                    let pdu_size = i32::from_le_bytes(buf_sz);
                    let name_len = i32::from_le_bytes(buf_nl);
                    //12..12+namelen: roboname
                    let mut robo_name = String::new();
                    for i in 0..name_len as usize {
                        let index = i + 12;
                        robo_name.push(buf[index] as char);
                    }
                    robo_name.push('\0');
                    //12+namelen..bufsize: buffer
                    let head_off = 12 + name_len as usize;
                    let ret = write_asset_pub_pdu(robo_name, channel_id, &buf[head_off..], pdu_size as usize);
                    assert!(ret == true);
                  }
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

pub fn send_all_subscriber(socket: &UdpSocket)
{
    let mut buf: [u8; ASSET_PACKET_MAX_SIZE] = [0; ASSET_PACKET_MAX_SIZE];
    let mut map = ASSET_SUB_PDU_CHANNELS.lock().unwrap();
    for (_real_id, pdu) in map.iter_mut() {
        if pdu.method_type == "UDP" {
            let result = api::asset_read_pdu(
                pdu.asset_name.as_ptr() as *const c_char, 
                pdu.robo_name.as_ptr() as *const c_char, 
                pdu.channel_id, 
                buf.as_mut_ptr() as *mut c_char, 
                pdu.pdu_size as i32);
            if result {
                send_one_subscriber(socket, pdu, pdu.channel_id, &buf, pdu.pdu_size as usize);
            }    
        }
    }
}


fn send_one_subscriber(socket: &UdpSocket, pdu: &mut AssetSubPduType, channel_id: i32, data: &[u8], size: usize)
{
    let name_len = pdu.robo_name.len();
    //let mut map = ASSET_SUB_PDU_CHANNELS.lock().unwrap();
    //let pdu: &mut AssetSubPduType = map.get_mut(&channel_id).unwrap();
    //0..3: channel id
    //4..7: bufsize
    //8..12: namelen
    //12..12+namelen: roboname
    let buf_ch = i32::to_le_bytes(channel_id);
    let buf_sz = i32::to_le_bytes(size as i32);
    let buf_nl = i32::to_le_bytes(name_len as i32);
    let buf = pdu.buffer.as_mut_slice();
    for i in 0..4 {
        buf[i] = buf_ch[i as usize];
        buf[i + 4] = buf_sz[i as usize];
        buf[i + 8] = buf_nl[i as usize];
    }
    for i in 0..name_len {
        buf[i + 12] = pdu.robo_name.as_bytes()[i];
    }
    let off = 12 + name_len;
    for i in 0..size {
        buf[i + off] = data[i];
    }
    
    socket.send_to(&pdu.buffer, pdu.options.udp_ip_port.clone()).expect("couldn't send data");
}

pub fn create_publisher_udp_socket(udp_ip_port: &String) -> UdpSocket
{
    println!("OPEN SENDER UDP PORT={}", udp_ip_port);
    let socket = UdpSocket::bind(udp_ip_port).unwrap();
    socket
}
