extern crate rosc;

use clap::Args;
use log::*;
use rosc::OscPacket;
use std::net::{SocketAddrV4, UdpSocket};
use std::str::FromStr;

#[derive(Args, Default)]
pub struct ReceiveOptions {
    #[arg(long = "host", default_value_t = String::from("127.0.0.1"))]
    pub host: String,

    #[arg(long = "port", default_value_t = 12345)]
    pub port: usize,
}

pub fn receive_osc(options: &ReceiveOptions) {
    // let args: Vec<String> = env::args().collect();
    // let usage = format!("Usage: {} RECEIVE_IP:RECEIVE_PORT", &args[0]);
    // if args.len() < 2 {
    //     println!("{}", &usage);
    //     panic!()
    // }
    // let receive_address = get_addr_from_arg(&args[1]);
    let receive_url = format!("{}:{}", options.host, options.port);
    info!("Will listen at {}", &receive_url);

    let socket = UdpSocket::bind(&receive_url).expect("Error binding udp socket");
    // println!("Listening to {}", receive_address);
    //

    let mut receive_buffer = [0u8; rosc::decoder::MTU];

    loop {
        match socket.recv_from(&mut receive_buffer) {
            Ok((size, addr)) => {
                println!("Received packet (length {}) from: {}", size, addr);
                let packet =
                    rosc::decoder::decode(&receive_buffer[..size]).expect("Error decoding packet");
                handle_packet(packet);
            }
            Err(e) => {
                println!("Error receiving from socket: {}", e);
                break;
            }
        }
    }
}

fn handle_packet(packet: OscPacket) {
    match packet {
        OscPacket::Message(msg) => {
            let arg_list = match msg.args {
                Some(args) => format!("{:?}", args),
                None => String::from("zero"),
            };
            println!("RCV OSC {} :: {}", msg.addr, arg_list);
        }
        OscPacket::Bundle(bundle) => {
            println!("OSC Bundle: {:?}", bundle);
        }
    }
}

fn get_addr_from_arg(arg: &str) -> SocketAddrV4 {
    SocketAddrV4::from_str(arg).expect("Invalid ip:port address")
}
