use std::env;
use std::error::Error;
use std::fs;
use std::process;

#[derive(Debug)]
struct Config {
    query: String,
    file: String,
}
impl Config {
    fn new(query: String, file: String) -> Config {
        Config { query: query, file }
    }

    pub fn parse(args: &[String]) -> Result<Config, &str> {
        if args.len() < 2 {
            return Err("insufficient arguments");
        }
        let query = &args[0].clone();
        let file = &args[1];
        Ok(Config {
            query: query.to_string(),
            file: file.to_string(),
        })
    }
}
fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if let Err(e) = run(load_config(args)) {
        println!("failed to run {}", e);
        process::exit(1);
    }
}

fn load_config(args: Vec<String>) -> Config {
    Config::parse(&args).unwrap_or_else(|er| {
        println!("failed to parse : {}", er);
        process::exit(1);
    })
   
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.file)?;
    print!("file content is \n{}", &contents);
    Ok(())
}
