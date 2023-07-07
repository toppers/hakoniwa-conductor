use std::env;
pub mod client;
pub mod loader;
pub mod hako;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <conductor-config> <robot-config>", args[0]);
        std::process::exit(1);
    }
    let conductor_config_path: &String = &args[1];
    let robot_config_path: &String = &args[2];
    match loader::load_conductor_config(conductor_config_path) {
        Ok(conductor_config) => {
            loader::show_conductor_config(&conductor_config);
            if let Err(e) = client::start_service(conductor_config, &robot_config_path).await {
                eprintln!("Error: client::strat_service() {}", e);
                std::process::exit(1);
            }        
        },
        Err(err) => {
            eprintln!("Failed to load data: {:?}", err);
            std::process::exit(1);
        }
    }

    Ok(())
}
