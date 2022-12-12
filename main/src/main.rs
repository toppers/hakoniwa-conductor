//extern crate link_cplusplus;
//extern crate libc;
use std::os::raw::c_longlong;

#[macro_use]
extern crate chan;
extern crate chan_signal;
use chan_signal::Signal;
use std::env;

//#[link(name = "fmt", kind = "static")]
#[link(name = "spdlog", kind = "dylib")]
#[link(name = "stdc++", kind = "dylib")]
#[link(name = "shakoc", kind = "static")]
extern "C" {
    fn hako_master_init() -> bool;
    fn hako_master_execute() -> bool;
    fn hako_master_set_config_simtime(max_delay_time_usec: c_longlong, delta_time_usec: c_longlong);
    //fn hako_master_get_max_deltay_time_usec() -> i64;
    //fn hako_master_get_delta_time_usec() -> i64;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Usage: {} <delta_msec> <max_delay_msec>", args[0]);
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

    println!("delta_msec = {}", delta_msec);
    println!("max_delay_msec = {}", max_delay_msec);
    let delta_usec = delta_msec * 1000;
    let max_delay_usec = max_delay_msec * 1000;
    unsafe {
        hako_master_init();
        let arg1 :c_longlong = delta_usec;
        let arg2: c_longlong = max_delay_usec;
        hako_master_set_config_simtime(arg1, arg2);
    }

    let s = chan_signal::notify(&[Signal::INT, Signal::TERM]);
    loop {
        let do_something = chan::after_ms(delta_msec as u32);
        chan_select! {
            s.recv() -> signal => {
                println!("signal={:?}", signal);
                std::process::exit(0);
            },
            do_something.recv() => {
                unsafe {
                    hako_master_execute();
                }
            }
        }
    }
}
