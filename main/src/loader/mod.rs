use serde::{Deserialize};
use std::fs::File;
use std::io::Read;

#[derive(Debug, Deserialize)]
struct RpcPduReader {
    #[serde(rename = "type")]
    pdu_type: String,
    org_name: String,
    name: String,
    channel_id: u32,
    pdu_size: u32,
    method_type: String,
}

#[derive(Debug, Deserialize)]
struct RpcPduWriter {
    #[serde(rename = "type")]
    pdu_type: String,
    org_name: String,
    name: String,
    channel_id: u32,
    pdu_size: u32,
    write_cycle: u32,
    method_type: String,
}
#[derive(Debug, Deserialize)]
struct Robot {
    name: String,
    rpc_pdu_readers: Vec<RpcPduReader>,
    rpc_pdu_writers: Vec<RpcPduWriter>,
}

#[derive(Debug, Deserialize)]
pub struct RobotConfig {
    #[serde(rename = "robots")]
    robots: Vec<Robot>,
}

#[derive(Debug, Deserialize)]
pub struct ConductorConfig {
    pub asset_name: String,
    pub core_ipaddr: String,
    pub core_portno: i32,
    pub delta_msec: i64,
    pub max_delay_msec: i64,
    pub udp_server_port: i32,
    pub udp_sender_port: i32,
    pub mqtt_portno: i32
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
    println!("Conductor udp_server_port: {}", conductor_config.udp_server_port);
    println!("Conductor udp_sender_port: {}", conductor_config.udp_sender_port);
    println!("Conductor mqtt_portno: {}", conductor_config.mqtt_portno);
    println!("-------------------");
}
