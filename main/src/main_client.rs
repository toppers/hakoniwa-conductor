#[macro_use]
extern crate chan;
extern crate chan_signal;
use chan_signal::Signal;
use std::env;
//use std::net::UdpSocket;
//use paho_mqtt as mqtt;

//internal modules
pub mod hako;
pub mod client;
pub mod loader;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <conductor-config> <robot-config>", args[0]);
        std::process::exit(1);
    }
    let conductor_config_path: &String = &args[1];
    let robot_config_path: &String = &args[2];
    let mut delta_usec: i64 = 1000 * 10;
    //let mut max_delay_usec = 1000 * 100;
    match loader::load_conductor_config(conductor_config_path) {
        Ok(conductor_config) => {
            loader::show_conductor_config(&conductor_config);
            delta_usec = conductor_config.delta_msec * 1000;
            //max_delay_usec = conductor_config.max_delay_msec * 1000;
            //hako::api::master_init(max_delay_usec, delta_usec);
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

    let s = chan_signal::notify(&[Signal::INT, Signal::TERM]);
    std::thread::spawn(move || {
        let delta_msec = delta_usec as u32 / 1000;
        loop {
            let do_something = chan::after_ms(delta_msec as u32);
            chan_select! {
                s.recv() -> signal => {
                    println!("signal={:?}", signal);
                    std::process::exit(0);
                },
                do_something.recv() => {

                }

            }
        }
    });

    //future.await?;    
    Ok(())
}
