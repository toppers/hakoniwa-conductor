//use tonic::{ transport::Server, Request, Response, Status};
//use tokio::sync::mpsc;
//use tokio_stream::wrappers::ReceiverStream;
use tonic::transport::{Endpoint, Uri};
use crate::{loader::{
    ConductorConfig, 
    RobotConfig,
    load_robot_config,
    show_robot_config
}, client::hakoniwa::NormalReply};

pub mod hakoniwa {
    tonic::include_proto!("hakoniwa");
}
//use crate::hako;

use hakoniwa::{
    core_service_client:: { CoreServiceClient },
    AssetInfo,
    ErrorCode,
    //AssetInfoList, SimStatReply, 
    //SimulationStatus,
    //SimulationTimeSyncOutputFile,
    //AssetNotification, 
    //AssetNotificationReply,
    //AssetNotificationEvent,
    //NotifySimtimeRequest, NotifySimtimeReply,
    //CreatePduChannelRequest, CreatePduChannelReply,
    SubscribePduChannelRequest, SubscribePduChannelReply
};
pub async fn start_service(conductor_config: ConductorConfig, robot_config_path: &String) -> Result<(), Box<dyn std::error::Error>> 
{
    let uri = format!("http://{}:{}", conductor_config.core_ipaddr.clone(), conductor_config.core_portno.clone()).parse::<Uri>()?;
    let endpoint = Endpoint::from(uri);
    let channel = endpoint.connect().await?;


    // Create a client using the channel
    let mut client: CoreServiceClient<tonic::transport::Channel> = CoreServiceClient::new(channel);

    // Create an AssetInfo message
    //TODO
    //0. server向けには、自分のアセット名で、登録する
    //1. 自分のアセット名は、外部定義ファイルから取得する。
    //2. 全SUBSCRチャネルは、外部定義ファイルから取得する。
    //3. 全CREATEチャネルは、外部定義ファイルから取得する。
    //4. 全CREATEチャネルを登録する(publishチャネル)
    //4. 全SUBSCRチャネルを登録する(subscribeチャネル)
    let asset_info = AssetInfo {
        name: conductor_config.asset_name.clone(),
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

    match load_robot_config(&robot_config_path) {
        Ok(config) => { 
            show_robot_config(&config);
            initialize_readers(&mut client, &conductor_config, &config).await?;
        },
        Err(err) => {
            eprintln!("Failed to load data: {:?}", err);
            std::process::exit(1);
        }
    }


    Ok(())
}

async fn initialize_readers(client: &mut CoreServiceClient<tonic::transport::Channel>, conductor_config: &ConductorConfig, robot_config: &RobotConfig) -> Result<(), Box<dyn std::error::Error>> 
{
    //TODO api::create_pdu_lchannel
    //TODO rpc::subscribe_pdu_channel

    for robot in &robot_config.robots {
        for reader in &robot.rpc_pdu_readers {
            let request = SubscribePduChannelRequest {
                asset_name: conductor_config.asset_name.clone(),
                channel_id: reader.channel_id as i32,
                pdu_size: reader.pdu_size as i32,
                listen_udp_ip_port: conductor_config.udp_sender_ip_port.clone(),
                method_type: reader.method_type.clone(),
                robo_name: robot.name.clone()
            };
            println!("Subscribe Pdu Channel Robot Name: {} Channel: {}", robot.name, reader.channel_id);
            let response = client.subscribe_pdu_channel(request).await?;
            let reply: &SubscribePduChannelReply = response.get_ref();
            println!("SubscribePduChannel response: {:?}", reply);
            if reply.ercd() != ErrorCode::Ok {
                return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Can not SubscribePduChannel"))); 
            }
            //TODO something
        }
    }
    Ok(())
}