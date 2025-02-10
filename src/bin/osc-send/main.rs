extern crate rosc;

use rosc::{encoder, OscMessage, OscPacket, OscType};
use std::env;
use std::io;
use std::net::{SocketAddrV4, UdpSocket};
use std::str::FromStr;

fn main() {
    let args: Vec<String> = env::args().collect();
    let usage = format!("Usage: {} HOST_IP:HOST_PORT DEST_IP:DEST_PORT", &args[0]);
    // println!("args length {}", args.len());
    if args.len() != 2 && args.len() != 3 {
        println!("{}", usage);
        panic!()
    }

    let host_addr: SocketAddrV4 = if args.len() == 3 {
        get_addr_from_arg(&args[1])
    } else {
        get_addr_from_arg("0.0.0.0:8080")
    };

    let dest_addr: SocketAddrV4 = if args.len() == 2 {
        get_addr_from_arg(&args[1])
    } else {
        get_addr_from_arg(&args[2])
    };

    let socket = UdpSocket::bind(host_addr).expect("Error binding udp socket");

    println!("Will send to {} from host {}", dest_addr, host_addr);

    loop {
        let mut command = String::new();

        io::stdin()
            .read_line(&mut command)
            .expect("Could not read that command");

        match command.as_str().trim() {
            "" => {
                println!("You didn't type anything");
            }
            _ => {
                send_message(&socket, dest_addr, command.as_str().trim());
            }
        };
    }
}

fn send_message(
    socket: &std::net::UdpSocket,
    destination_address: std::net::SocketAddrV4,
    command: &str,
) {
    let parts = command.split_whitespace().collect::<Vec<&str>>();
    println!("parts: {:?}", parts);
    let mut args: Vec<OscType> = Vec::new();

    for part in &parts[1..parts.len()] {
        // skip first element
        println!("part: {:?}", part);

        args.push(auto_type_arg(part));
    }

    println!("final args: {:?}", args);

    let osc_address = parts[0];
    println!("will send {} args to address {}", args.len(), osc_address);

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

fn get_addr_from_arg(arg: &str) -> SocketAddrV4 {
    SocketAddrV4::from_str(arg).expect("Invalid ip:port address")
}
