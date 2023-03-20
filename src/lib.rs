use std::error::Error;
use std::fs;
use std::process;

#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub  file: String,
}
impl Config {
    pub fn new(query: String, file: String) -> Config {
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

pub fn load_config(args: Vec<String>) -> Config {
    Config::parse(&args).unwrap_or_else(|er| {
        println!("failed to parse : {}", er);
        process::exit(1);
    })
   
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.file)?;
    print!("file content is \n{}", &contents);
    Ok(())
}
