use chan;
use chan_signal;
use chan_signal::Signal;
use chan::chan_select;
use std::net::UdpSocket;
use paho_mqtt as mqtt;
use crate::hako;
extern crate lazy_static;
extern crate once_cell;
use std::{sync::Mutex};
use once_cell::sync::Lazy;

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

use hakoniwa::{
    core_service_client:: { CoreServiceClient },
    AssetInfo,
    ErrorCode,
    //AssetInfoList, SimStatReply, 
    //SimulationStatus,
    AssetNotification, 
    //AssetNotificationReply,
    AssetNotificationEvent,
    //NotifySimtimeRequest, NotifySimtimeReply,
    CreatePduChannelRequest, CreatePduChannelReply,
    SubscribePduChannelRequest, SubscribePduChannelReply
};
pub enum SimulationState {
    Stopped,
    Runnable,
    Running,
    Stopping,
    Terminated,
}

pub struct ClientSimStatus {
    pub master_time: i64,
    pub event: AssetNotificationEvent,
    pub state: SimulationState,
    pub is_pdu_created: bool,
    pub is_simulation_mode: bool,
    pub is_pdu_sync_mode: bool,
}

pub static CLIENT_SIM_STATUS: Lazy<Mutex<ClientSimStatus>> = Lazy::new(|| {
    Mutex::new(ClientSimStatus { 
        master_time: 0,
        event: AssetNotificationEvent::None,
        state: SimulationState::Terminated,
        is_pdu_created: false,
        is_simulation_mode: false,
        is_pdu_sync_mode: false
    })
});

pub async fn start_service(conductor_config: ConductorConfig, robot_config_path: &String) -> Result<(), Box<dyn std::error::Error>> 
{
    //TODO
    //hako::api::master_init(max_delay_usec, delta_usec);

    let uri = format!("http://{}:{}", conductor_config.core_ipaddr.clone(), conductor_config.core_portno.clone()).parse::<Uri>()?;
    let endpoint = Endpoint::from(uri);
    let channel = endpoint.connect().await?;

    // Create a client using the channel
    let mut client: CoreServiceClient<tonic::transport::Channel> = CoreServiceClient::new(channel);

    // Create an AssetInfo message
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
            initialize_writers(&mut client, &conductor_config, &config).await?;
        }
        Err(err) => {
            eprintln!("Failed to load data: {:?}", err);
            std::process::exit(1);
        }
    }
    let future = {
        let thread_conductor_config: ConductorConfig = conductor_config.clone();
        event_monitor(thread_conductor_config)
    };
    tokio::spawn(async move {
        if let Err(err) = future.await {
            eprintln!("Error in event_monitor: {:?}", err);
            std::process::exit(1);
        }
    });
    //CREATE UDP SOCKET
    let socket: Option<UdpSocket> = Some(hako::method::udp::create_publisher_udp_socket(&conductor_config.udp_sender_ip_port));
    hako::method::udp::activate_server(&conductor_config.udp_server_ip_port);
    //MQTT SERVER
    let mut cli: Option<mqtt::Client> = None;
    if conductor_config.mqtt_portno > 0 {
        hako::method::mqtt::set_mqtt_url(conductor_config.core_ipaddr.clone(), conductor_config.mqtt_portno);
        cli = hako::method::mqtt::create_mqtt_publisher();
    }
    //EXEC SIMULATION
    let delta_msec: u32 = conductor_config.delta_msec as u32;
    let s = chan_signal::notify(&[Signal::INT, Signal::TERM]);
    loop {
        let do_something = chan::after_ms(delta_msec as u32);
        chan_select! {
            s.recv() -> signal => {
                println!("signal={:?}", signal);
                std::process::exit(0);
            },
            do_something.recv() => {
                //TODO update status
                //if hako::api::master_execute() {
                if true {
                    match socket {
                        Some(ref _n) => {
                            //TODO
                            //hako::method::udp::send_all_subscriber(socket.as_ref().unwrap());
                        },
                        None => ()
                    }
                    //println!("is_enabled={}", hako::method::mqtt::is_enabled() );
                    if hako::method::mqtt::is_enabled() {
                        //println!("start mqtt send1");
                        match cli {
                            Some(ref _n) => {
                                //println!("start mqtt send2");
                                hako::method::mqtt::publish_mqtt_topics(_n);
                            },
                            None => ()
                        }
                    }
                }
            }
        }
    }
}

async fn event_monitor(conductor_config: ConductorConfig) -> Result<(), Box<dyn std::error::Error>> 
{
    let uri = format!("http://{}:{}", conductor_config.core_ipaddr.clone(), conductor_config.core_portno.clone()).parse::<Uri>()?;
    let endpoint = Endpoint::from(uri);
    let channel = endpoint.connect().await?;

    // Create a client using the channel
    let mut client: CoreServiceClient<tonic::transport::Channel> = CoreServiceClient::new(channel);

    // Create an AssetInfo message
    let asset_info = AssetInfo {
        name: conductor_config.asset_name.clone(),
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


async fn initialize_readers(client: &mut CoreServiceClient<tonic::transport::Channel>, conductor_config: &ConductorConfig, robot_config: &RobotConfig) -> Result<(), Box<dyn std::error::Error>> 
{
    //TODO api::create_pdu_lchannel
    for robot in &robot_config.robots {
        for reader in &robot.rpc_pdu_readers {
            let request = SubscribePduChannelRequest {
                asset_name: conductor_config.asset_name.clone(),
                channel_id: reader.channel_id as i32,
                pdu_size: reader.pdu_size as i32,
                listen_udp_ip_port: conductor_config.udp_server_ip_port.clone(),
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
            let result = hako::pdu::create_asset_pub_pdu(
                conductor_config.asset_name.clone(), 
                robot.name.clone(), 
                reader.channel_id as i32, 
                reader.pdu_size as i32, 
                reader.method_type.clone());
            if result == false {
                return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Can not create_asset_sub_pdu"))); 
            }
        }
    }
    Ok(())
}


async fn initialize_writers(client: &mut CoreServiceClient<tonic::transport::Channel>, conductor_config: &ConductorConfig, robot_config: &RobotConfig) -> Result<(), Box<dyn std::error::Error>> 
{
    //TODO api::create_pdu_lchannel
    for robot in &robot_config.robots {
        for writer in &robot.rpc_pdu_writers {
            let request = CreatePduChannelRequest {
                asset_name: conductor_config.asset_name.clone(),
                channel_id: writer.channel_id as i32,
                pdu_size: writer.pdu_size as i32,
                method_type: writer.method_type.clone(),
                robo_name: robot.name.clone()
            };
            println!("Create Pdu Channel Robot Name: {} Channel: {}", robot.name, writer.channel_id);
            let response = client.create_pdu_channel(request).await?;
            let reply: &CreatePduChannelReply = response.get_ref();
            println!("CreatePduChannel response: {:?}", reply);
            if reply.ercd() != ErrorCode::Ok {
                return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Can not CreatePduChannel"))); 
            }
            let result = hako::pdu::create_asset_sub_pdu(
                                conductor_config.asset_name.clone(), 
                                robot.name.clone(), 
                                writer.channel_id as i32, 
                                writer.pdu_size as i32, 
                                response.get_ref().port.to_string(), 
                                writer.method_type.clone());
            if result == false {
                return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Can not create_asset_sub_pdu"))); 
            }
        }
    }
    Ok(())
}