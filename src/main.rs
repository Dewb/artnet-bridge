use structopt::StructOpt;
use std::net::{Ipv4Addr, UdpSocket, SocketAddr, ToSocketAddrs};
use artnet_protocol::{ArtCommand, PollReply};
use std::str::FromStr;
use clap_verbosity_flag::Verbosity;
use log::{info, debug, trace, Level};

extern crate pretty_env_logger;

mod kinet;
mod utils;


#[derive(Debug, StructOpt)]
/// Map Art-Net universes to KiNET PDS endpoints
struct Cli {
    /// The network address to listen on 
    #[structopt(short = "l", long = "listen")]
    artnet_address: String,
    /// The network address to send KiNET from   
    #[structopt(short = "k", long = "kinet")]
    kinet_address: String,
    /// The KiNET PDS addresses to send to
    #[structopt(short = "p", long = "pds", required(true))]
    pds_addresses: Vec<String>,
    // Output verbosity control
    #[structopt(flatten)]
    verbose: Verbosity,
}

fn main() {

    let mut args = Cli::from_args();
    args.verbose.set_default(Some(Level::Warn));
    
    pretty_env_logger::formatted_timed_builder()
        .filter(None, args.verbose.log_level().unwrap().to_level_filter())
        .init();
    
    let mut short_name: [u8; 18] = [0; 18];
    let mut long_name: [u8; 64] = [0; 64];

    let default_short_name = "ArtNet/KiNETBridge";
    let default_long_name = "ArtNet/KiNET Bridge v0.1.0";
    short_name.copy_from_slice(&default_short_name.as_bytes()[..18]);
    long_name[..26].copy_from_slice(&default_long_name.as_bytes()[..]);

    info!("Listening for Art-Net packets on {}", args.artnet_address);
    info!("Transmitting KiNET on {}", args.kinet_address);
    info!("Mapping universes to the following addresses:");
    info!("{:?}", args.pds_addresses);
    
    let artnet_socket = UdpSocket::bind((&args.artnet_address[..], 6454)).unwrap();
    let kinet_socket = UdpSocket::bind((&args.kinet_address[..], 6038)).unwrap();

    let pds_addrs: Vec<SocketAddr> = args.pds_addresses.iter().map(|addr_string| {
       (&addr_string[..], 6038).to_socket_addrs().unwrap().next().unwrap()
    }).collect();
    
    loop {
        let mut buffer = [0u8; 1024];
        let (length, addr) = artnet_socket.recv_from(&mut buffer).unwrap();
        let command = ArtCommand::from_buffer(&buffer[..length]).unwrap();
        
        match command {
            ArtCommand::Poll(poll) => {
                debug!("Received Art-Net poll command {:?}", poll);
                
                let command = ArtCommand::PollReply(
                    Box::new( 
                        PollReply {
                            address: Ipv4Addr::from_str(&args.artnet_address).unwrap(),
                            port: 6454,
                            num_ports: utils::clone_into_array(&args.pds_addresses.len().to_le_bytes()[..2]),
                            short_name: short_name,
                            long_name: long_name,
                            ..utils::default_poll_reply()
                        }
                    )
                );
                let bytes = command.into_buffer().unwrap();
                artnet_socket.send_to(&bytes, &addr).unwrap();
            },
            ArtCommand::PollReply(_reply) => {
            },
            ArtCommand::Output(output) => {
                debug!("Received Art-Net output command for subnet {:?} of length {:?}", output.subnet, output.length);
                trace!("{:?}", output);

                let mut kinet_output = kinet::Output::default();
                kinet_output.data.copy_from_slice(&output.data[..512]);
                let bytes = kinet_output.serialize();
                
                debug!("Sending KiNET output packet to {:?}", pds_addrs[0]);
                trace!("{:?}", bytes);
                
                kinet_socket.send_to(&bytes, &pds_addrs[0]).unwrap();
            },
            _ => {
                debug!("Received unhandled Art-Net command");
                trace!("{:?}", command);
            }
        }
    }
}