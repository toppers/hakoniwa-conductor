use std::net::UdpSocket;

pub mod pdu;
pub mod api;

pub fn asset_create_pdu_channel(asset_name: String, channel_id: i32, pdu_size: i32) -> bool
{
    let result = pdu::create_asset_pub_pdu(asset_name, channel_id, pdu_size);
    if result == false {
        return false;
    }
    let result = api::asset_create_pdu_channel(channel_id, pdu_size);
    if result == false {
        pdu::remove_asset_pub_pdu(channel_id);
        return false;
    }
    else {
        return true;
    }
}
