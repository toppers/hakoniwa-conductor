
use tonic::transport::{Endpoint, Uri};

pub mod hakoniwa {
    tonic::include_proto!("hakoniwa");
}
use hakoniwa::{
    core_service_client:: { CoreServiceClient },
    AssetInfo,
    ErrorCode,
    SimulationStatus,
    NormalReply,
    AssetNotification, 
    AssetNotificationReply,
    AssetNotificationEvent,
    NotifySimtimeRequest, NotifySimtimeReply,
    CreatePduChannelRequest, CreatePduChannelReply,
    SubscribePduChannelRequest, SubscribePduChannelReply
};
use std::{sync::Mutex};
use once_cell::sync::Lazy;
use tonic::Response;

#[derive(Debug, Clone)]
pub enum SimulationState {
    Stopped,
    Runnable,
    Running,
    Stopping,
    Terminated,
}
#[derive(Debug, Clone)]
pub struct ClientSimStatus {
    pub master_time: i64,
    pub event: AssetNotificationEvent,
    pub state: SimulationState,
    pub is_pdu_created: bool,
    pub is_simulation_mode: bool,
    pub is_pdu_sync_mode: bool,
}

static CLIENT_SIM_STATUS: Lazy<Mutex<ClientSimStatus>> = Lazy::new(|| {
    Mutex::new(ClientSimStatus { 
        master_time: 0,
        event: AssetNotificationEvent::None,
        state: SimulationState::Terminated,
        is_pdu_created: false,
        is_simulation_mode: false,
        is_pdu_sync_mode: false
    })
});

pub async fn create_client(core_ipaddr: &String, portno: i32) -> Result<CoreServiceClient<tonic::transport::Channel>, Box<dyn std::error::Error>> 
{
    let uri = format!("http://{}:{}", core_ipaddr.clone(), portno).parse::<Uri>()?;
    let endpoint = Endpoint::from(uri);
    let channel = endpoint.connect().await?;

    // Create a client using the channel
    Ok(CoreServiceClient::new(channel))
}
pub async fn asset_register(client: &mut CoreServiceClient<tonic::transport::Channel>, asset_name: &String) -> Result<(), Box<dyn std::error::Error>> 
{
    let asset_info = AssetInfo {
        name: asset_name.clone(),
    };

    // Send the register request
    let request = tonic::Request::new(asset_info);
    let response = client.register(request).await?;

    // Process the response
    let reply: &NormalReply = response.get_ref();
    println!("Register response: {:?}", reply);
    if reply.ercd() != ErrorCode::Ok {
        return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Can not register asset"))); 
    }
    Ok(())
}
pub async fn asset_notification_feedback(client: &mut CoreServiceClient<tonic::transport::Channel>, asset_name: &String, ev: AssetNotificationEvent, result: bool) -> Result<(), Box<dyn std::error::Error>> 
{
    let mut ercd = ErrorCode::Ok;
    if result == false {
        ercd = ErrorCode::Inval;
    }
    let asset_info = AssetInfo {
        name: asset_name.clone(),
    };
    let request = AssetNotificationReply {
        event: ev.into(),
        asset: Some(asset_info),
        ercd: ercd.into()
    };

    // Send the register request
    let req = tonic::Request::new(request);
    let response = client.asset_notification_feedback(req).await?;
    if response.get_ref().ercd() != ErrorCode::Ok {
        return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "notification feedback response error"))); 
    }

    Ok(())
}
fn get_state(status: SimulationStatus) -> SimulationState {
    match status {
        SimulationStatus::StatusStopped => SimulationState::Stopped,
        SimulationStatus::StatusStopping => SimulationState::Stopping,
        SimulationStatus::StatusRunnable => SimulationState::Runnable,
        SimulationStatus::StatusRunning => SimulationState::Running,
        _ => SimulationState::Terminated,
    }
}
pub async fn asset_notify_simtime(client: &mut CoreServiceClient<tonic::transport::Channel>, asset_name: &String, asset_time: i64, is_read_pdu_done: bool, is_write_pdu_done: bool) -> Result<ClientSimStatus, Box<dyn std::error::Error>> 
{
    let req = NotifySimtimeRequest {
        asset: Some(AssetInfo {
            name: asset_name.clone()
        }),
        asset_time: asset_time,
        is_read_pdu_done: is_read_pdu_done,
        is_write_pdu_done: is_write_pdu_done
    };
    let res: Response<NotifySimtimeReply> = client.notify_simtime(req).await?;
    if res.get_ref().ercd() != ErrorCode::Ok {
        return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "notify simtime response error"))); 
    }
    let mut client_sim_status = CLIENT_SIM_STATUS.lock().unwrap();
    client_sim_status.master_time = res.get_ref().master_time;
    client_sim_status.is_pdu_created = res.get_ref().is_pdu_created ;
    client_sim_status.is_pdu_sync_mode = res.get_ref().is_pdu_sync_mode;
    client_sim_status.is_simulation_mode = res.get_ref().is_simulation_mode;
    client_sim_status.state = get_state(res.get_ref().status());
    Ok(client_sim_status.clone())
}


pub fn get_simevent() -> AssetNotificationEvent
{
    let mut client_sim_status = CLIENT_SIM_STATUS.lock().unwrap();
    let ret = client_sim_status.event;
    client_sim_status.event = AssetNotificationEvent::None;
    return ret;
}
pub async fn event_monitor(asset_name: String, core_ipaddr: String, portno: i32) -> Result<(), Box<dyn std::error::Error>> 
{
    // Create a client using the channel
    let mut client: CoreServiceClient<tonic::transport::Channel> = create_client(&core_ipaddr, portno).await?;

    // Create an AssetInfo message
    let asset_info = AssetInfo {
        name: asset_name.clone(),
    };

    let request = tonic::Request::new(asset_info);
    let response = client.asset_notification_start(request).await?;

    // ストリーミングレスポンスを受け取る
    let mut stream = response.into_inner();
    loop {
        let notification: Option<AssetNotification> = stream.message().await?;
        match notification  {
            Some(notification) => {
                match notification.event() {
                    AssetNotificationEvent::Start => {
                        let mut client_sim_status = CLIENT_SIM_STATUS.lock().unwrap();
                        client_sim_status.event = AssetNotificationEvent::Start;
                    }
                    AssetNotificationEvent::Stop => {
                        let mut client_sim_status = CLIENT_SIM_STATUS.lock().unwrap();
                        client_sim_status.event = AssetNotificationEvent::Stop;
                    }
                    AssetNotificationEvent::Reset => {
                        let mut client_sim_status = CLIENT_SIM_STATUS.lock().unwrap();
                        client_sim_status.event = AssetNotificationEvent::Reset;
                    }
                    AssetNotificationEvent::Error => {
                        let mut client_sim_status = CLIENT_SIM_STATUS.lock().unwrap();
                        client_sim_status.event = AssetNotificationEvent::Error;
                    }
                    AssetNotificationEvent::Heartbeat => {
                        /* nothing to do */
                        println!("Heartbeat");
                    }
                    AssetNotificationEvent::None => {
                        /* nothing to do */
                        println!("NONE");
                    }
                }
            }
            None => {}
        }
    }
}
