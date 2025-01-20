use structopt::StructOpt;
use std::net::{Ipv4Addr, UdpSocket};
use artnet_protocol::{ArtCommand, PollReply};
use std::str::FromStr;

use log::{error, info, debug, trace};
use anyhow::Error;

extern crate pretty_env_logger;
extern crate serde_json;
extern crate bincode;

mod config;
mod kinet;
mod utils;

fn main() -> Result<(), Error> {

    // Load configuration from command line
    let cli_args = config::UserConfiguration::from_args();

    let mut file_args = config::UserConfiguration::default();
    if cli_args.config_file.is_some() {
        let file_path = cli_args.config_file.clone().unwrap();
        file_args = config::UserConfiguration::from_file(file_path)?;
    }

    let cfg = config::Configuration::from_user_configs(cli_args, file_args)?;

    pretty_env_logger::formatted_timed_builder()
        .filter(None, cfg.get_log_level().unwrap().to_level_filter())
        .init();

    let mut short_name: [u8; 18] = [0; 18];
    let mut long_name: [u8; 64] = [0; 64];

    let default_short_name = "ArtNet/KiNETBridge";
    let default_long_name = "ArtNet/KiNET Bridge v0.1.0";
    short_name.copy_from_slice(&default_short_name.as_bytes()[..18]);
    long_name[..26].copy_from_slice(&default_long_name.as_bytes()[..]);

    info!("Listening for Art-Net packets on {}", cfg.artnet_receive_ip);
    info!("Transmitting KiNET on {}", cfg.kinet_send_ip);
    info!("Mapping Art-Net to the following KiNET destinations:");
    for mapping in cfg.kinet_destinations.values() {
        info!("{:?}", mapping);
    }
        
    let artnet_socket = 
        UdpSocket::bind((&cfg.artnet_receive_ip[..], 6454))
        .expect("Could not bind to Art-Net address.");
    let kinet_socket = 
        UdpSocket::bind((&cfg.kinet_send_ip[..], 6038))
        .expect("Could not bind to KiNET address.");
    
    loop {
        let mut buffer = [0u8; 1024];
        let (length, addr) = artnet_socket.recv_from(&mut buffer)?;
        let command = ArtCommand::from_buffer(&buffer[..length])?;
        
        match command {
            ArtCommand::Poll(poll) => {
                debug!("Received Art-Net poll command {:?}", poll);
                
                let command = ArtCommand::PollReply(
                    Box::new( 
                        PollReply {
                            address: Ipv4Addr::from_str(&cfg.artnet_receive_ip)?,
                            port: 6454,
                            short_name: short_name,
                            long_name: long_name,
                            ..utils::default_poll_reply()
                        }
                    )
                );
                match utils::send_artnet_command(command, &artnet_socket, &addr) {
                    Err(e) => { error!("{:?}", e); },
                    Ok(()) => {}
                }
            },
            ArtCommand::PollReply(_reply) => {
            },
            ArtCommand::Output(output) => {

                let artnet_port_address = u16::from(output.port_address);

                let artnet_universe = (artnet_port_address & 0x0F) as u8;
                let artnet_subnet   = ((artnet_port_address & 0xF0) >> 4) as u8;
                let artnet_network  = ((artnet_port_address & 0x7F00) >> 8) as u8;

                debug!("Received Art-Net output command for net/subnet/universe {:?}:{:?}:{:?} with length {:?}", 
                    artnet_network, artnet_subnet, artnet_universe, length);
                trace!("{:?}", output);

                match cfg.kinet_destinations.get(&artnet_port_address) {
                    None => {
                        debug!("No KiNET destination specified for this Art-Net output");
                    },
                    Some(destination) => {
                        let payload : &Vec<u8> = output.data.as_ref();
                        if destination.kinet_port == 0 {
                            let mut dmx_out_msg = kinet::DmxOut::default();
                            dmx_out_msg.data[..length as usize].copy_from_slice(&payload[..length as usize]);
                            match bincode::serialize(&dmx_out_msg) {
                                Err(e) => { error!("{:?}", e); },
                                Ok(bytes) => {
                                    debug!("Sending KiNET DmxOut packet to {:?}", destination.kinet_address);
                                    trace!("{:?}", bytes);
                                    match kinet_socket.send_to(&bytes, &destination.kinet_socket_addr) {
                                        Err(e) => { error!("{:?}", e); },
                                        Ok(_count) => {}
                                    }
                                }
                            }
                        } else {
                            let mut port_out_msg = kinet::PortOut::default();
                            port_out_msg.port = destination.kinet_port;
                            port_out_msg.data[..length as usize].copy_from_slice(&payload[..length as usize]);
                            match bincode::serialize(&port_out_msg) {
                                Err(e) => { error!("{:?}", e); },
                                Ok(bytes) => {
                                    debug!("Sending KiNET PortOut packet to {:?} port {:?}", destination.kinet_address, destination.kinet_port);
                                    trace!("{:?}", bytes);
                                    
                                    match kinet_socket.send_to(&bytes, &destination.kinet_socket_addr) {
                                        Err(e) => { error!("{:?}", e); },
                                        Ok(_count) => {}
                                    }
                                }
                            }
                        }
                    }
                }
            },
            _ => {
                debug!("Received unhandled Art-Net command");
                trace!("{:?}", command);
            }
        }
    }
}