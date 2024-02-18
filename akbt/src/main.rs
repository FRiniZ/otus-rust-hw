mod backup;
mod errors;
mod helpers;
mod protos;

use std::fmt::Display;

use clap::{Parser, Subcommand};
use log::info;

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
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    Backup,
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

fn main() {
    env_logger::init();

    let _cli = Args::parse();

    info!("Another Kafka Backup Tool starting...");

    info!("BOOTSTRAP_SERVERS: {}", _cli.bootstrap_servers);
    info!("TOPIC: {}", _cli.topic);
    info!("Command: {}", _cli.cmd);

    let result = match _cli.cmd {
        Commands::Backup => backup::backup(_cli.bootstrap_servers, _cli.topic, _cli.file),
        Commands::Restore => todo!(),
    };

    info!("Result:{:?}", result);

    info!("Bye bye");
}
