use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Usage: {} <delta_msec> <max_delay_msec>", args[0]);
        std::process::exit(1);
    }
    let delta_msec: u64 = args[1].parse::<u64>().expect("ERROR invalid delta_msec");
    let max_delay_msec: u64 = args[2].parse::<u64>().expect("ERROR invalid max_delay_msec");

    println!("delta_msec = {}", delta_msec);
    println!("max_delay_msec = {}", max_delay_msec);

    std::process::exit(0);
}
