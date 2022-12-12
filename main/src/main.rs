extern crate link_cplusplus;

#[macro_use]
extern crate chan;
extern crate chan_signal;
use std::env;
use chan_signal::Signal;

#[link(name="spdlog", kind="static")]
#[link(name="shakoc", kind="static")]
extern {
    fn hako_master_init() -> bool;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Usage: {} <delta_msec> <max_delay_msec>", args[0]);
        std::process::exit(1);
    }
    let delta_msec: u32 = match args[1].parse::<u32>() {
        Ok(n) => n,
        Err(e) => {
            println!("ERROR delta_msec {}", e);
            std::process::exit(1);
        }
    };
    let max_delay_msec: u32 = match args[2].parse::<u32>() {
        Ok(n) => n,
        Err(e) => {
            println!("ERROR max_delay_msec: {}", e);
            std::process::exit(1);
        }
    };

    println!("delta_msec = {}", delta_msec);
    println!("max_delay_msec = {}", max_delay_msec);
    unsafe {
        hako_master_init();
    }

    let s = chan_signal::notify(&[Signal::INT, Signal::TERM]);
    loop {
        let do_something = chan::after_ms(delta_msec);
        chan_select! {
            s.recv() -> signal => {
                println!("signal={:?}", signal);
                std::process::exit(0);
            },
            do_something.recv() => {
                println!("do something");
            }
        }
    }
}
