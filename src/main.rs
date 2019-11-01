extern crate rosc;

use rosc::{OscPacket, OscMessage, OscType, encoder};
use std::io;
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

        // match sock.recv_from(&mut receive_buffer) {
        //     Ok((size, addr)) => {
        //         println!("Received packet (length {}) from: {}", size, addr);
        //         let packet = rosc::decoder::decode(&receive_buffer[..size]).unwrap();
        //         handle_packet(packet);
        //     }
        //     Err(e) => {
        //         println!("Error receiving from socket: {}", e);
        //         break;
        //     }
        // }

        let mut command = String::new();

        io::stdin().read_line(&mut command)

            .expect("Could not read that command");

        match command.as_str().trim() {
            "" => {
                println!("You didn't type anything");
            } 
            _ => {
                send_message(&socket, send_address, command.as_str().trim());
            }
        };
        


    }
}

fn send_message(socket: &std::net::UdpSocket, destination_address: std::net::SocketAddrV4, command: &str) {
    let parts = command.split_whitespace().collect::<Vec<&str>>();
    println!("parts: {:?}", parts);
    let mut args: Vec<OscType> = Vec::new();
    
    for part in &parts[1..parts.len()]  { // skip first element
        println!("part: {:?}", part);

        args.push(auto_type_arg(part));
    }

    println!("final args: {:?}", args);

    let osc_address = parts[0];
    println!("will send {} args to address {}", args.len(), osc_address);

    let buffer = encoder::encode(&OscPacket::Message(OscMessage {
        addr: osc_address.to_string(),
        args: Some(args)
    })).unwrap();

    match socket.send_to(&buffer, destination_address) {
        Ok(usize) => println!("OK, {} bytes sent", usize),
        Err(_) => panic!("Error sending message")
    }

}

fn auto_type_arg (part: &str) -> OscType {
    
    match part {
        part if !part.parse::<i32>().is_err() => OscType::Int(part.parse::<i32>().unwrap()),
        part if !part.parse::<f32>().is_err() => OscType::Float(part.parse::<f32>().unwrap()),
        _ => OscType::String(part.to_string())
    }

    // let osc_int = match part.parse::<i32>() {
    //     Ok(value) => Some(OscType::Int(value)),
    //     Err(e) => None
    // };

    // let osc_float = match part.parse::<f32>() {
    //     Ok(value) => Some(OscType::Float(value)),
    //     Err(e) => None
    // };

    // if osc_int.is_some() {
    //     osc_int.unwrap()
    // } else if osc_float.is_some() {
    //     osc_float.unwrap()
    // } else {
    //     OscType::String(part.to_string())
    // }

}


fn get_addr_from_arg(arg: &str) -> SocketAddrV4 {
    match SocketAddrV4::from_str(arg) {
        Ok(address) => address,
        Err(_) => panic!("Invalid ip:port address")
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