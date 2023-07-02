//use tonic::{ transport::Server, Request, Response, Status};
//use tokio::sync::mpsc;
//use tokio_stream::wrappers::ReceiverStream;
use tonic::transport::{Endpoint, Uri};
use crate::loader::{
    ConductorConfig, 
    RobotConfig,
    load_robot_config,
    show_robot_config
};

pub mod hakoniwa {
    tonic::include_proto!("hakoniwa");
}
//use crate::hako;

use hakoniwa::{
    core_service_client:: { CoreServiceClient },
    AssetInfo,
    //ErrorCode, , NormalReply,
    //AssetInfoList, SimStatReply, 
    //SimulationStatus,
    //SimulationTimeSyncOutputFile,
    //AssetNotification, 
    //AssetNotificationReply,
    //AssetNotificationEvent,
    //NotifySimtimeRequest, NotifySimtimeReply,
    //CreatePduChannelRequest, CreatePduChannelReply,
    //SubscribePduChannelRequest, SubscribePduChannelReply
};
pub async fn start_service(conductor_config: ConductorConfig, robot_config_path: &String) -> Result<(), Box<dyn std::error::Error>> 
{
    match load_robot_config(&robot_config_path) {
        Ok(config) => show_robot_config(&config),
        Err(err) => {
            eprintln!("Failed to load data: {:?}", err);
            std::process::exit(1);
        }
    }
    let uri = format!("http://{}:{}", conductor_config.core_ipaddr, conductor_config.core_portno).parse::<Uri>()?;
    let endpoint = Endpoint::from(uri);
    let channel = endpoint.connect().await?;


    // Create a client using the channel
    let mut client = CoreServiceClient::new(channel);

    // Create an AssetInfo message
    //TODO
    //0. server向けには、自分のアセット名で、登録する
    //1. 自分のアセット名は、外部定義ファイルから取得する。
    //2. 全SUBSCRチャネルは、外部定義ファイルから取得する。
    //3. 全CREATEチャネルは、外部定義ファイルから取得する。
    //4. 全CREATEチャネルを登録する(publishチャネル)
    //4. 全SUBSCRチャネルを登録する(subscribeチャネル)
    let asset_info = AssetInfo {
        name: conductor_config.asset_name,
    };

    // Send the register request
    let request = tonic::Request::new(asset_info);
    let response = client.register(request).await?;

    // Process the response
    let reply = response.get_ref();
    println!("Register response: {:?}", reply);

    Ok(())
}
