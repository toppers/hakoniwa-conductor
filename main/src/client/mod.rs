use tonic::{ transport::Server, Request, Response, Status};
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::transport::{Endpoint, Uri};

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
pub async fn start_service(ip: &String, port: &String) -> Result<(), Box<dyn std::error::Error>> 
{
    let uri = format!("http://{}:{}", ip, port).parse::<Uri>()?;
    let endpoint = Endpoint::from(uri);
    let channel = endpoint.connect().await?;


    // Create a client using the channel
    let mut client = CoreServiceClient::new(channel);

    // Create an AssetInfo message
    let asset_info = AssetInfo {
        name: "SampleAsset".to_string(),
    };

    // Send the register request
    let request = tonic::Request::new(asset_info);
    let response = client.register(request).await?;

    // Process the response
    let reply = response.get_ref();
    println!("Register response: {:?}", reply);

    Ok(())
}
