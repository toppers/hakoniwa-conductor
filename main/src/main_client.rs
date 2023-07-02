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
    if args.len() != 6 {
        eprintln!("Usage: {} <delta_msec> <max_delay_msec> <ipaddr> <port> <channel-json>", args[0]);
        std::process::exit(1);
    }
    let delta_msec: i64 = match args[1].parse::<i64>() {
        Ok(n) => n,
        Err(e) => {
            eprintln!("ERROR delta_msec {}", e);
            std::process::exit(1);
        }
    };
    let max_delay_msec: i64 = match args[2].parse::<i64>() {
        Ok(n) => n,
        Err(e) => {
            eprintln!("ERROR max_delay_msec: {}", e);
            std::process::exit(1);
        }
    };
    let ip: &String = &args[3];
    let port: &String = &args[4];
    let channel_json_path: &String = &args[5];
    match loader::load_data(channel_json_path) {
        Ok(config) => loader::show_data(config),
        Err(err) => {
            eprintln!("Failed to load data: {:?}", err);
            std::process::exit(1);
        }
    }
    println!("delta_msec = {}", delta_msec);
    println!("max_delay_msec = {}", max_delay_msec);
    let delta_usec: i64 = delta_msec * 1000;
    let max_delay_usec: i64 = max_delay_msec * 1000;
    //hako::api::master_init(max_delay_usec, delta_usec);
    if let Err(e) = client::start_service(ip, port).await {
        eprintln!("Error: {}", e);
    }
    let s = chan_signal::notify(&[Signal::INT, Signal::TERM]);
    std::thread::spawn(move || {
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
