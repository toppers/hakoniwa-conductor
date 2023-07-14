use serde::{Deserialize};
use std::fs::File;
use std::io::Read;

#[derive(Debug, Deserialize)]
pub struct RpcPduReader {
    #[serde(rename = "type")]
    pub pdu_type: String,
    pub org_name: String,
    pub name: String,
    pub channel_id: u32,
    pub pdu_size: u32,
    pub method_type: String,
}

#[derive(Debug, Deserialize)]
pub struct RpcPduWriter {
    #[serde(rename = "type")]
    pub pdu_type: String,
    pub org_name: String,
    pub name: String,
    pub channel_id: u32,
    pub pdu_size: u32,
    pub write_cycle: u32,
    pub method_type: String,
}
#[derive(Debug, Deserialize)]
pub struct Robot {
    pub name: String,
    pub rpc_pdu_readers: Vec<RpcPduReader>,
    pub rpc_pdu_writers: Vec<RpcPduWriter>,
}

#[derive(Debug, Deserialize)]
pub struct RobotConfig {
    #[serde(rename = "robots")]
    pub robots: Vec<Robot>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ConductorConfig {
    pub asset_name: String,
    pub core_ipaddr: String,
    pub core_portno: i32,
    pub delta_msec: i64,
    pub max_delay_msec: i64,
    pub udp_server_ip_port: String,
    pub udp_sender_ip_port: String,
    pub mqtt_portno: i32,
    pub mqtt_pub_client_id: String,
    pub mqtt_sub_client_id: String
}

pub fn load_robot_config(filename: &str) -> Result<RobotConfig, Box<dyn std::error::Error>> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let config: RobotConfig = serde_json::from_str(&contents)?;
    Ok(config)
}

pub fn show_robot_config(robot_config: &RobotConfig)
{
    // データの使用例として、ロボットの一覧を表示する
    for robot in &robot_config.robots {
        println!("Robot Name: {}", robot.name);
        println!("RPC PDU Readers:");
        for reader in &robot.rpc_pdu_readers {
            println!("  - Type: {}", reader.pdu_type);
            println!("    Org Name: {}", reader.org_name);
            println!("    Name: {}", reader.name);
            // 他のフィールドも必要に応じて表示する
        }
        println!("RPC PDU Writers:");
        for writer in &robot.rpc_pdu_writers {
            println!("  - Type: {}", writer.pdu_type);
            println!("    Org Name: {}", writer.org_name);
            println!("    Name: {}", writer.name);
            // 他のフィールドも必要に応じて表示する
        }
        println!("-------------------");
    }
}


pub fn load_conductor_config(filename: &str) -> Result<ConductorConfig, Box<dyn std::error::Error>> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let config: ConductorConfig = serde_json::from_str(&contents)?;
    Ok(config)
}


pub fn show_conductor_config(conductor_config: &ConductorConfig)
{
    println!("Conductor asset_name: {}", conductor_config.asset_name);
    println!("Conductor core_ipaddr: {}", conductor_config.core_ipaddr);
    println!("Conductor core_portno: {}", conductor_config.core_portno);
    println!("Conductor delta_msec: {}", conductor_config.delta_msec);
    println!("Conductor max_delay_msec: {}", conductor_config.max_delay_msec);
    println!("Conductor udp_server_port: {}", conductor_config.udp_server_ip_port);
    println!("Conductor udp_sender_port: {}", conductor_config.udp_sender_ip_port);
    println!("Conductor mqtt_portno: {}", conductor_config.mqtt_portno);
    println!("Conductor mqtt_pub_client_id: {}", conductor_config.mqtt_pub_client_id);
    println!("Conductor mqtt_sub_client_id: {}", conductor_config.mqtt_sub_client_id);
    println!("-------------------");
}
