mod backup;
mod consumer;
mod counters;
mod errors;
mod gzip;
mod mbprocess;
mod protos;
mod restore;

use std::fmt::Display;
use std::process::ExitCode;

use clap::{Parser, Subcommand};
use log::info;
use std::env;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, env("BOOTSTRAP_SERVERS"))]
    bootstrap_servers: String,
    #[arg(short, long, env("TOPIC"))]
    topic: String,
    #[command(subcommand)]
    cmd: Commands,
    #[arg(short, long, env("FILE"))]
    file: String,
    ///Compression level <0-9>(none-the_best)
    #[arg(short, long, default_value = "0")]
    level: u32,
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    /// Backup topic to file
    Backup,
    /// Restore topic from file
    Restore,
}

impl Display for Commands {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Commands::Backup => write!(f, "Backup"),
            Commands::Restore => write!(f, "Restore"),
        }
    }
}

fn main() -> ExitCode {
    env_logger::init();

    let log_enabled = if let Ok(_) = env::var("RUST_LOG") {
        true
    } else {
        false
    };

    let c = Args::parse();

    info!("Another Kafka Backup Tool starting...");

    info!("BOOTSTRAP_SERVERS: {}", c.bootstrap_servers);
    info!("TOPIC: {}", c.topic);
    info!("Command: {}", c.cmd);

    let result = match c.cmd {
        Commands::Backup => {
            backup::backup(c.bootstrap_servers, c.topic, c.file, c.level, log_enabled)
        }
        Commands::Restore => restore::restore(c.bootstrap_servers, c.topic, c.file, log_enabled),
    };

    if result.is_err() {
        println!("{:?}", result.unwrap_err().to_string());
        return ExitCode::FAILURE;
    }
    ExitCode::SUCCESS
}
