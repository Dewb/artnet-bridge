use structopt::StructOpt;
use serde::Deserialize;
use log::Level;
use anyhow::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use anyhow::{anyhow, Result};
use std::collections::HashMap;
use std::net::{Ipv4Addr, SocketAddr, ToSocketAddrs};
use std::str::FromStr;

#[derive(Debug)]
pub struct KinetDestination {
    pub artnet_network: u16,
    pub artnet_subnet: u8,
    pub artnet_universe: u8,
    pub kinet_address: String,
    pub kinet_socket_addr: SocketAddr,
    pub kinet_universe: u8,
}

pub struct Configuration {
    pub artnet_receive_ip: String,
    pub kinet_send_ip: String,
    pub kinet_destinations: HashMap<u16, KinetDestination>,
    pub verbosity: i8,
}

#[derive(Debug, StructOpt, Deserialize, Default)]
/// Map Art-Net universes to KiNET PDS endpoints
pub struct UserConfiguration {
    /// The IPv4 network address where Art-Net packets will be received
    #[structopt(short = "a", display_order = 1)]
    pub artnet_receive_ip: Option<String>,
    /// The IPv4 network address that KiNET packets should be sent from   
    #[structopt(short = "k", display_order = 2)]
    pub kinet_send_ip: Option<String>,
    /// Map a single Art-Net universe data to a KiNET destination. Map-string should consist of an Art-Net source universe and 
    /// a KiNET destination IPv4 address, with optional output universe, all separated by colons.
    /// Art-Net source universes can be specified with just a universe, or a network, subnet, and universe.
    /// 1:0:15:10.0.0.1:3 would send listen for Art-Net output for network 1, subnet 0, universe 15,
    /// and send it to the KiNET PDS at 10.0.0.1 for output on universe/channel 3.
    /// If any network/subnet/universe values are not provided, they will be assumed to be 0, so the following are all valid:
    /// -m 10.0.0.4 -m 3:192.168.10.100 -m 1:4:13:10.0.1.4 -m 192.168.0.15:10 -m 1:1:10.0.0.2:2
    #[structopt(short = "m", long = "mapping", value_name = "map-string", display_order = 3)]
    pub mappings: Option<Vec<String>>,
    /// Path to a file containing configuration options. All command-line options can be specified in the config file;
    /// command-line options will override options in file where there's a conflict. 
    #[structopt(short = "f", long = "file")]
    #[serde(skip)]
    pub config_file: Option<String>,
    /// Make output more verbose. Add -v for debugging info, add -vv for even more detailed message tracing.
    #[structopt(long, short = "v", parse(from_occurrences))]
    #[serde(default)]
    pub verbose: i8,
    /// Make output less verbose. Add -q to only show warnings and errors, -qq to only show errors, and -qqq to silence output completely.
    #[structopt(long, short = "q", parse(from_occurrences), conflicts_with = "verbose")]
    #[serde(default)]
    pub quiet: i8,
}

impl Configuration {
    pub fn get_log_level(&self) -> Option<Level> {
        match self.verbosity {
            std::i8::MIN..=-1 => None,
            0 => Some(log::Level::Error),
            1 => Some(log::Level::Warn),
            2 => Some(log::Level::Info),
            3 => Some(log::Level::Debug),
            4..=std::i8::MAX => Some(log::Level::Trace),
        }
    }

    pub fn from_user_configs(cli_config: UserConfiguration, file_config: UserConfiguration) -> Result<Self, Error> {
        // Return a configuration object we can use from both the CLI and optional config file.

         let artnet_address = match cli_config.artnet_receive_ip {
            None => match file_config.artnet_receive_ip {
                None => return Err(anyhow!("No Art-Net listening address specified.")),
                Some(addr) => addr,
            },
            Some(addr) => addr,
        };

        let kinet_address = match cli_config.kinet_send_ip {
            None => match file_config.kinet_send_ip {
                None => return Err(anyhow!("No KiNET output address specified.")),
                Some(addr) => addr,
            },
            Some(addr) => addr,
        };

        let mut mappings = vec!();
        mappings.extend(cli_config.mappings.unwrap_or_default());
        mappings.extend(file_config.mappings.unwrap_or_default());
        
        if mappings.len() == 0 {
            return Err(anyhow!("No KiNET destination mappings specified."));
        }

        mappings.sort_unstable();
        mappings.dedup();

        let default_verbosity: i8 = 2;
        let verbosity = default_verbosity 
            + cli_config.verbose - cli_config.quiet
            + file_config.verbose - file_config.quiet;
      
        let config = Configuration {
            artnet_receive_ip: artnet_address,
            kinet_send_ip: kinet_address,
            kinet_destinations: mappings_to_destinations(mappings)?,
            verbosity: verbosity,
        };

        return Ok(config);
    }
}

impl UserConfiguration {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<UserConfiguration, Error> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
    
        let cfg = serde_json::from_reader(reader)?;
    
        Ok(cfg)
    }
}


fn mappings_to_destinations(mappings: Vec<String>) -> Result<HashMap<u16, KinetDestination>> {
    let mut destination_map = HashMap::new();

    for dest in mappings {
        let mut tokens: Vec<&str> = dest.split(':').collect();
        let kinet_address: String;
        let kinet_universe: u8;

        let item = tokens.pop().unwrap_or_default();
        match Ipv4Addr::from_str(item) {
            Ok(_) => {
                kinet_address = item.to_string();
                kinet_universe = 0;
            },
            Err(_) => {
                kinet_universe = match item.parse::<u8>() {
                    Ok(u) => u,
                    Err(_) => {
                        return Err(anyhow!("Could not understand {} as a KiNET destination address or universe", item));
                    }
                };
                let item = tokens.pop().unwrap_or_default();
                match Ipv4Addr::from_str(item) {
                    Ok(_) => {
                        kinet_address = item.to_string();
                    },
                    Err(_) => {
                        return Err(anyhow!("Could not understand {} as a KiNET destination address", item));
                    }
                }
            }
        }

        let kinet_socket_addr = match (&kinet_address[..], 6038).to_socket_addrs() {
            Ok(mut addresses) => {
                match addresses.next() {
                    Some(address) => { address },
                    None => {
                        return Err(anyhow!("Could not create socket address for {}", kinet_address));
                    }
                }
            },
            Err(_) => {
                return Err(anyhow!("Could not create socket address for {}", kinet_address));
            }
        };
        
        let artnet_universe = match tokens.pop() {
            Some(val) => {
                match val.parse::<u8>() {
                    Ok(n) => n,
                    _ => {
                        return Err(anyhow!("Could not understand {} as an Art-Net universe", val));
                    }
                }
            },
            None => 0,
        };
        let artnet_subnet = match tokens.pop() {
            Some(val) => {
                match val.parse::<u8>() {
                    Ok(n) => n,
                    _ => {
                        return Err(anyhow!("Could not understand {} as an Art-Net subnet", val));
                    }
                }
            },
            None => 0,
        };
        let artnet_network = match tokens.pop() {
            Some(val) => {
                match val.parse::<u16>() {
                    Ok(n) => n,
                    _ => {
                        return Err(anyhow!("Could not understand {} as an Art-Net network", val));
                    }
                }
            },
            None => 0,
        };
        
        let combined_address = 
            (artnet_universe & 0x0F) as u16 + 
            ((artnet_subnet & 0xF0) << 4) as u16 +
            ((artnet_network & 0x7F00) << 8);

        destination_map.insert(combined_address, KinetDestination {
            artnet_network,
            artnet_subnet,
            artnet_universe,
            kinet_address,
            kinet_socket_addr,
            kinet_universe,
        });
    }

    return Ok(destination_map);
}