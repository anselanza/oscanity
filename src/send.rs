extern crate rosc;

use clap::Args;
use log::*;
use rosc::{encoder, OscMessage, OscPacket, OscType};
use std::io;
use std::net::{SocketAddrV4, UdpSocket};
use std::str::FromStr;

#[derive(Args, Default)]
pub struct SendOptions {
    #[arg(long = "host", default_value_t = String::from("127.0.0.1"))]
    pub host: String,

    #[arg(
        long = "port",
        default_value_t = 12345,
        help = "Port that the destination will be listening on (target port)"
    )]
    pub destination_port: usize,

    #[arg(
        long = "port.bind",
        default_value_t = 54321,
        help = "Port to bind as source locally; usually need not be changed"
    )]
    pub src_bound_port: usize,
}

pub fn send_osc(options: &SendOptions) {
    let bind_address =
        SocketAddrV4::from_str(&format!("{}:{}", options.host, options.src_bound_port))
            .expect("failed to convert string to address");
    let dest_address =
        SocketAddrV4::from_str(&format!("{}:{}", options.host, options.destination_port))
            .expect("failed to convert string to address");

    info!(
        "Will send to host {}; from bound port {} => destination port {}",
        options.host, options.src_bound_port, options.destination_port
    );
    let socket = UdpSocket::bind(bind_address).expect("Error binding udp socket");

    loop {
        let mut command = String::new();

        io::stdin()
            .read_line(&mut command)
            .expect("Could not read that command");

        match command.as_str().trim() {
            "" => {
                error!("You didn't type anything");
            }
            _ => {
                send_message(&socket, &dest_address, command.as_str().trim());
            }
        };
    }
}

fn send_message(
    socket: &std::net::UdpSocket,
    destination_address: &std::net::SocketAddrV4,
    command: &str,
) {
    let parts = command.split_whitespace().collect::<Vec<&str>>();
    debug!("parts: {:?}", parts);
    let mut args: Vec<OscType> = Vec::new();

    for part in &parts[1..parts.len()] {
        // skip first element
        debug!("part: {:?}", part);

        args.push(auto_type_arg(part));
    }

    println!("final args: {:?}", args);

    let osc_address = parts[0];
    debug!("will send {} args to address {}", args.len(), osc_address);

    let buffer = encoder::encode(&OscPacket::Message(OscMessage {
        addr: osc_address.to_string(),
        args: Some(args),
    }))
    .expect("Error encoding message");

    match socket.send_to(&buffer, destination_address) {
        Ok(usize) => println!("OK, {} bytes sent", usize),
        Err(_) => panic!("Error sending message"),
    }
}

fn auto_type_arg(part: &str) -> OscType {
    match part {
        part if part.parse::<i32>().is_ok() => OscType::Int(part.parse::<i32>().unwrap()),
        part if part.parse::<f32>().is_ok() => OscType::Float(part.parse::<f32>().unwrap()),
        _ => OscType::String(part.to_string()),
    }
}
