use std::env;
use std::process;
use grep::{Config,*};

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if let Err(e) = run(load_config(args)) {
        println!("failed to run {}", e);
        process::exit(1);
    }
}
