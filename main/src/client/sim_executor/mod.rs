
use crate::client::rpc_client;
use crate::client::rpc_client::hakoniwa::core_service_client::CoreServiceClient;
use crate::client::rpc_client::hakoniwa::AssetNotificationEvent;
use crate::hako;


async fn server_event_handling(client: &mut CoreServiceClient<tonic::transport::Channel>, asset_name: &String) -> Result<bool, Box<dyn std::error::Error>>
{
    let event: AssetNotificationEvent = rpc_client::get_simevent();
    match event {
        AssetNotificationEvent::Start => {
            let ret = hako::api::simevent_start();
            rpc_client::asset_notification_feedback(client, asset_name, event, ret).await?;
        }
        AssetNotificationEvent::Stop => {
            let ret = hako::api::simevent_stop();
            rpc_client::asset_notification_feedback(client, asset_name, event, ret).await?;
        }
        AssetNotificationEvent::Reset => {
            let ret = hako::api::simevent_reset();
            rpc_client::asset_notification_feedback(client, asset_name, event, ret).await?;
        }
        AssetNotificationEvent::None => {
            //nothing to do
        }
        AssetNotificationEvent::Heartbeat => {
            //nothing to do
        }
        AssetNotificationEvent::Error => {
            //nothing to do
        }
    }
    Ok(true)
}

fn asset_do_callback(asset_name: &String)-> Result<bool, Box<dyn std::error::Error>>
{
    let event: hako::api::SimulationAssetEventType = hako::api::asset_get_event(asset_name.clone());
    match event {
        hako::api::SimulationAssetEventType::Start => {
            hako::api::asset_start_feedback(asset_name.clone(), true);
        }
        hako::api::SimulationAssetEventType::Stop => {
            hako::api::asset_stop_feedback(asset_name.clone(), true);
        }
        hako::api::SimulationAssetEventType::Reset => {
            hako::api::asset_reset_feedback(asset_name.clone(), true);
        }
        hako::api::SimulationAssetEventType::None => {
            //nothing to do
        }
        hako::api::SimulationAssetEventType::Invalid => {
            //nothing to do
        }
        hako::api::SimulationAssetEventType::Error => {
            //nothing to do
        }
    }
    Ok(true)
}
pub async fn execute(client: &mut CoreServiceClient<tonic::transport::Channel>, asset_name: &String) -> Result<bool, Box<dyn std::error::Error>>
{
    server_event_handling(client, asset_name).await?;
    asset_do_callback(&asset_name)?;

    let is_sim_mode = hako::api::asset_is_simulation_mode();
    let is_read_pdu_done = is_sim_mode;
    let is_write_pdu_done = is_sim_mode;
    let asset_time = hako::api::asset_get_worldtime();
    let status = rpc_client::asset_notify_simtime(client, asset_name, asset_time, is_read_pdu_done, is_write_pdu_done).await?;

    if status.state != rpc_client::SimulationState::Running {
        //シミュレーション開始していないので、まだPDUデータを送信できない。
        return Ok(false);
    }
    else if status.is_pdu_created == false {
        //サーバー側はまだPDUチャネルが未完成なので、まだPDUデータを送信できない。
        return Ok(false);
    }
    else if status.is_pdu_sync_mode {
        //サーバ側の準備ができたので、PUDデータを通知して
        //サーバー側の PDU sync mode を解消させる必要がある。
        return Ok(true);
    }
    else if status.is_simulation_mode == false {
        //サーバー側がシミュレーション開始できる状態になるまで待つ。
        //一方で、PDUは送信しても良いでしょう。
        return Ok(true);
    }
    //is_pdu_created == true
    //is_pdu_sync_mode == false
    //is_simulation_mode == true
    //check asset time
    hako::api::asset_notify_simtime(asset_name.clone(), status.master_time);
    Ok(hako::api::master_execute())
}

