use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Usage: {} <delta_msec> <max_delay_msec>", args[0]);
        std::process::exit(1);
    }
    let delta_msec: u64 = match args[1].parse::<u64>() {
        Ok(n) => n,
        Err(e) => {
            println!("ERROR delta_msec {}", e);
            std::process::exit(1);
        },
    };
    let max_delay_msec: u64 = match args[2].parse::<u64>() {
        Ok(n) => n,
        Err(e) => {
            println!("ERROR max_delay_msec: {}", e);
            std::process::exit(1);
        },
    };

    println!("delta_msec = {}", delta_msec);
    println!("max_delay_msec = {}", max_delay_msec);

    std::process::exit(0);
}
