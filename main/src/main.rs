#[macro_use]
extern crate chan;
extern crate chan_signal;
use chan_signal::Signal;
use std::env;
use std::net::UdpSocket;

//internal modules
pub mod hako;
pub mod server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 && args.len() != 6 {
        println!("Usage: {} <delta_msec> <max_delay_msec> <ipaddr>:<port> [<udp_server_port>] [<udp_sender_port>]", args[0]);
        std::process::exit(1);
    }
    let delta_msec: i64 = match args[1].parse::<i64>() {
        Ok(n) => n,
        Err(e) => {
            println!("ERROR delta_msec {}", e);
            std::process::exit(1);
        }
    };
    let max_delay_msec: i64 = match args[2].parse::<i64>() {
        Ok(n) => n,
        Err(e) => {
            println!("ERROR max_delay_msec: {}", e);
            std::process::exit(1);
        }
    };
    let v: Vec<&str> = args[3].split(':').collect();
    let ipaddr: String = String::from(v[0]);
    let mut socket: Option<UdpSocket> = None;
    if args.len() == 6 {
        let udp_server_ip_port: String = ipaddr.clone() + ":" + &args[4];
        hako::method::udp::activate_server(&udp_server_ip_port);
        let udp_sender_ip_port: String = ipaddr + ":" + &args[5];
        socket = Some(hako::method::udp::create_publisher_udp_socket(&udp_sender_ip_port));
    }

    println!("delta_msec = {}", delta_msec);
    println!("max_delay_msec = {}", max_delay_msec);
    let delta_usec: i64 = delta_msec * 1000;
    let max_delay_usec: i64 = max_delay_msec * 1000;
    hako::api::master_init(max_delay_usec, delta_usec);
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
                    if hako::api::master_execute() {
                        match socket {
                            Some(ref _n) => {
                                hako::method::udp::send_all_subscriber(socket.as_ref().unwrap());
                            },
                            None => ()
                        }
                    }
                }
            }
        }    
    });

    let future = server::start_service(&args[3]);
    future.await?;
    Ok(())
}
