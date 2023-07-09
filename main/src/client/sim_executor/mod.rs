
use crate::client::rpc_client;
use crate::client::rpc_client::hakoniwa::core_service_client::CoreServiceClient;
use crate::hako;

pub async fn execute(client: &mut CoreServiceClient<tonic::transport::Channel>, asset_name: &String) -> Result<bool, Box<dyn std::error::Error>>
{
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
