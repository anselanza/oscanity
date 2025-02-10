use clap::{Parser, Subcommand};
use env_logger::Env;
use log::*;
use oscanity::{
    receive::{receive_osc, ReceiveOptions},
    send::{send_osc, SendOptions},
};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,

    #[arg(long = "loglevel",default_value_t=String::from("info"))]
    pub log_level: String,
}

#[derive(Subcommand)]
enum Commands {
    Receive(ReceiveOptions),
    Send(SendOptions),
}

fn main() {
    let cli = Cli::parse();

    env_logger::Builder::from_env(Env::default().default_filter_or(&cli.log_level))
        // .filter_module("rumqttc", log::LevelFilter::Warn)
        .init();

    debug!("Debug log level is enabled; could be verbose");

    match &cli.command {
        Commands::Receive(receive_options) => receive_osc(receive_options),
        Commands::Send(send_options) => send_osc(send_options),
    }
}
