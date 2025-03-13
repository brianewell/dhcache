mod cli;
use cli::{Cli,Parser};
use log::{info,warn,error};

fn main() {
    env_logger::init();
    let cli = Cli::parse();

    warn!("Application started.");

    println!("Debug: bind={}", cli.bind);
    println!("Debug: port={}", cli.port);
    println!("Debug: workers={}", cli.workers);

    for i in cli.cache_parameters {
        println!("Debug: parameter {:?}", i);
    }
}
