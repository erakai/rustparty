use std::env;
use std::process;

use rustparty::*;
use colored::Colorize;

struct Config {
    ip: String,
    port: usize,
    is_client: bool,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        match &args[1][..] {
            "host" => {
                if args.len() != 3 {
                    return Err("incorrect argument count");
                }
                let port = args[2].parse().unwrap();
                Ok(Config { ip: String::new(), port, is_client: false })
            },
            "join" => {
                if args.len() != 4 {
                    return Err("incorrect argument count");
                }
                let ip = args[2].clone();
                let port = args[3].parse().unwrap();
                Ok(Config { ip, port, is_client: true})
            },
            _ => {
                Err("not host or join")
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|e| {
        eprintln!("Argument Error: {}. Please either run \"{}\" or \"{}\".", 
                 e, "rustparty host <port>".to_string().yellow().bold(), 
                 "rustparty join <ip> <port>".to_string().yellow().bold());
        process::exit(1);
    });

    println!("Beginning rustparty...");
    // thread::sleep(Duration::from_millis(1500)); 

    rustparty(config);
}

fn rustparty(conf: Config) {
    if conf.is_client {
        run_client(conf.ip, conf.port);
    }  else {
        run_server(conf.port);
    }
}
