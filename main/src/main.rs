#[macro_use]
extern crate chan;
extern crate chan_signal;
use chan_signal::Signal;
use std::env;

//internal modules
pub mod hako;
pub mod server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 && args.len() != 4 {
        println!("Usage: {} <delta_msec> <max_delay_msec> [<ipaddr:udp_server_port>]", args[0]);
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
    if args.len() == 4 {
        hako::pdu::activate_server(&args[3]);
    }

    println!("delta_msec = {}", delta_msec);
    println!("max_delay_msec = {}", max_delay_msec);
    let delta_usec: i64 = delta_msec * 1000;
    let max_delay_usec: i64 = max_delay_msec * 1000;
    hako::master_init(max_delay_usec, delta_usec);
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
                    hako::master_execute();
                }
            }
        }    
    });

    let future = server::start_service();
    future.await?;
    Ok(())
}
