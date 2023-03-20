pub mod pdu;
pub mod api;
pub mod method;

pub fn asset_create_pdu_channel(asset_name: String, channel_id: i32, pdu_size: i32, method_type: String) -> bool
{
    let result = pdu::create_asset_pub_pdu(asset_name.clone(), channel_id, pdu_size, method_type);
    if result == false {
        return false;
    }
    let result = api::asset_create_pdu_lchannel(asset_name, channel_id, pdu_size);
    if result == false {
        pdu::remove_asset_pub_pdu(channel_id);
        return false;
    }
    else {
        return true;
    }
}
