use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;

#[derive(Debug, Deserialize)]
struct RpcPduReader {
    #[serde(rename = "type")]
    pdu_type: String,
    org_name: String,
    name: String,
    class_name: String,
    class_path: String,
    conv_class_name: String,
    conv_class_path: String,
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
    class_name: String,
    class_path: String,
    conv_class_name: String,
    conv_class_path: String,
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

pub fn load_data(filename: &str) -> Result<RobotConfig, Box<dyn std::error::Error>> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let config: RobotConfig = serde_json::from_str(&contents)?;
    Ok(config)
}

pub fn show_data(robot_config: RobotConfig)
{
    // データの使用例として、ロボットの一覧を表示する
    for robot in robot_config.robots {
        println!("Robot Name: {}", robot.name);
        println!("RPC PDU Readers:");
        for reader in robot.rpc_pdu_readers {
            println!("  - Type: {}", reader.pdu_type);
            println!("    Org Name: {}", reader.org_name);
            println!("    Name: {}", reader.name);
            // 他のフィールドも必要に応じて表示する
        }
        println!("RPC PDU Writers:");
        for writer in robot.rpc_pdu_writers {
            println!("  - Type: {}", writer.pdu_type);
            println!("    Org Name: {}", writer.org_name);
            println!("    Name: {}", writer.name);
            // 他のフィールドも必要に応じて表示する
        }
        println!("-------------------");
    }
}