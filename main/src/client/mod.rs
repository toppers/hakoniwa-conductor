use chan;
use chan_signal;
use chan_signal::Signal;
use chan::chan_select;
use std::net::UdpSocket;
use paho_mqtt as mqtt;
use crate::hako;
extern crate lazy_static;
extern crate once_cell;
pub mod rpc_client;
pub mod sim_executor;

use crate::{loader::{
    ConductorConfig, 
    RobotConfig,
    load_robot_config,
    show_robot_config
}};

pub mod hakoniwa {
    tonic::include_proto!("hakoniwa");
}
use crate::client::rpc_client::hakoniwa::core_service_client::CoreServiceClient;

use crate::client::rpc_client::hakoniwa::{
    ErrorCode,
    CreatePduChannelRequest, CreatePduChannelReply,
    SubscribePduChannelRequest, SubscribePduChannelReply
};

pub async fn start_service(conductor_config: ConductorConfig, robot_config_path: &String) -> Result<(), Box<dyn std::error::Error>> 
{
    hako::api::master_init(conductor_config.max_delay_msec * 1000, conductor_config.delta_msec * 1000);
    let mut client = rpc_client::create_client(&conductor_config.core_ipaddr.clone(), conductor_config.core_portno.clone()).await?;
    hako::api::asset_init();
    if hako::api::asset_register_polling(conductor_config.asset_name.clone()) {
        println!("INFO: asset_register_polling() success");
        rpc_client::asset_register(&mut client, &conductor_config.asset_name).await?;    
    }
    else {
        eprintln!("Failed to register asset: {}", conductor_config.asset_name.clone());
        std::process::exit(1);
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
        rpc_client::event_monitor(thread_conductor_config.asset_name.clone(), thread_conductor_config.core_ipaddr, thread_conductor_config.core_portno.clone())
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
                match sim_executor::execute(&mut client, &conductor_config.asset_name).await {
                    Ok(true) => {
                        match socket {
                            Some(ref _n) => {
                                hako::method::udp::send_all_subscriber(socket.as_ref().unwrap());
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
                    Ok(false) => {}
                    Err(e) => {
                        eprintln!("sim_executor::execute: {:?}", e);
                    }
                }
            }
        }
    }
}

async fn initialize_readers(client: &mut CoreServiceClient<tonic::transport::Channel>, conductor_config: &ConductorConfig, robot_config: &RobotConfig) -> Result<(), Box<dyn std::error::Error>> 
{
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
            let ret = hako::api::asset_create_pdu_lchannel(robot.name.clone(), reader.channel_id as i32, reader.pdu_size as i32);
            if ret == false {
                return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "asset_create_pdu_lchannel error"))); 
            }

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
    for robot in &robot_config.robots {
        for writer in &robot.rpc_pdu_writers {
            let request = CreatePduChannelRequest {
                asset_name: conductor_config.asset_name.clone(),
                channel_id: writer.channel_id as i32,
                pdu_size: writer.pdu_size as i32,
                method_type: writer.method_type.clone(),
                robo_name: robot.name.clone()
            };
            //let ret = hako::api::asset_create_pdu_lchannel(robot.name.clone(), writer.channel_id as i32, writer.pdu_size as i32);
            //if ret == false {
            //    return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "asset_create_pdu_lchannel error"))); 
            //}
            //println!("Create Pdu Channel Robot Name: {} Channel: {}", robot.name, writer.channel_id);
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