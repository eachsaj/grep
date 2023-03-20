use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();
    println!("input arguments{:?}",args);
}
