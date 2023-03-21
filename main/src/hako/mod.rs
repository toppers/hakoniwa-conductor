pub mod pdu;
pub mod api;
pub mod method;

pub fn asset_create_pdu_channel(asset_name: String, robo_name: String, channel_id: i32, pdu_size: i32, method_type: String) -> bool
{
    let result = api::asset_create_pdu_lchannel(robo_name.clone(), channel_id, pdu_size);
    if result == false {
        return false;
    }
    let result = pdu::create_asset_pub_pdu(asset_name.clone(), robo_name.clone(), channel_id, pdu_size, method_type);
    if result == false {
        return false;
    }
    else {
        return true;
    }
}
