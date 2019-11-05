extern crate rosc;

use rosc::{OscPacket, };
use std::env;
use std::net::{SocketAddrV4, UdpSocket};
use std::str::FromStr;

fn main() {
    let args: Vec<String> = env::args().collect();
    let usage = format!("Usage: {} RECEIVE_IP:RECEIVE_PORT SEND_IP:SEND_PORT", &args[0]);
    if args.len() < 3 {
        println!("{}", usage);
        panic!(usage)
    }
    let receive_address = get_addr_from_arg(&args[1]);
    let send_address = get_addr_from_arg(&args[2]);

    let socket = UdpSocket::bind(receive_address).unwrap();
    println!("Listening to {}, will send to {}", receive_address, send_address);

    let mut receive_buffer = [0u8; rosc::decoder::MTU];

    loop {

        match socket.recv_from(&mut receive_buffer) {
            Ok((size, addr)) => {
                println!("Received packet (length {}) from: {}", size, addr);
                let packet = rosc::decoder::decode(&receive_buffer[..size]).unwrap();
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
                Some(args) => {
                    format!("{:?}", args)
                }
                None => {
                   String::from("zero")
                }
            };
            println!("RCV OSC {} :: {}", msg.addr, arg_list);
        }
        OscPacket::Bundle(bundle) => {
            println!("OSC Bundle: {:?}", bundle);
        }
    }
}

fn get_addr_from_arg(arg: &str) -> SocketAddrV4 {
    match SocketAddrV4::from_str(arg) {
        Ok(address) => address,
        Err(_) => panic!("Invalid ip:port address")
    }
}