mod backup;
mod errors;
mod gzwriter;
mod protos;

use std::fmt::Display;
use std::process::ExitCode;

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
    #[arg(short, long, default_value = "0")]
    n_wrk: usize,
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

    let _cli = Args::parse();

    info!("Another Kafka Backup Tool starting...");

    info!("BOOTSTRAP_SERVERS: {}", _cli.bootstrap_servers);
    info!("TOPIC: {}", _cli.topic);
    info!("N_WRK: {}", _cli.n_wrk);
    info!("Command: {}", _cli.cmd);

    let result = match _cli.cmd {
        Commands::Backup => {
            backup::backup(_cli.n_wrk, _cli.bootstrap_servers, _cli.topic, _cli.file)
        }
        Commands::Restore => todo!(),
    };

    if result.is_err() {
        println!("{:?}", result.unwrap_err().to_string());
        return ExitCode::FAILURE;
    }
    ExitCode::SUCCESS
}
