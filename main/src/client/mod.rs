use tonic::{ transport::Server, Request, Response, Status};
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;

pub mod hakoniwa {
    tonic::include_proto!("hakoniwa");
}
use crate::hako;

use hakoniwa::{
    core_service_client:: { CoreServiceClient },
    ErrorCode, AssetInfo, NormalReply,
    AssetInfoList, SimStatReply, SimulationStatus,
    SimulationTimeSyncOutputFile,
    AssetNotification, AssetNotificationReply,
    AssetNotificationEvent,
    NotifySimtimeRequest, NotifySimtimeReply,
    CreatePduChannelRequest, CreatePduChannelReply,
    SubscribePduChannelRequest, SubscribePduChannelReply
};

pub async fn start_service(ip_port: &String) -> Result<(), Box<dyn std::error::Error>>
{
    //TODO
    //TODO create channel
    //TODO register asset
    //TODO monitor event
        //TODO wait start, reset, ...etc..
    Ok(())
}
