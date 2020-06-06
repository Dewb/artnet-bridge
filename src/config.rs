use structopt::StructOpt;
use serde::Deserialize;
use log::Level;
use anyhow::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use anyhow::{anyhow, Result};

pub struct Configuration {
    pub artnet_address: String,
    pub kinet_address: String,
    pub pds_addresses: Vec<String>,
    pub verbosity: i8,
}

#[derive(Debug, StructOpt, Deserialize, Default)]
/// Map Art-Net universes to KiNET PDS endpoints
pub struct UserConfiguration {
    /// The network address to listen on 
    #[structopt(short = "l", long = "listen")]
    pub artnet_address: Option<String>,
    /// The network address to send KiNET from   
    #[structopt(short = "k", long = "kinet")]
    pub kinet_address: Option<String>,
    /// The KiNET PDS addresses to send to
    #[structopt(short = "p", long = "pds")]
    pub pds_addresses: Option<Vec<String>>,
    /// Path to a file containing configuration options. All command-line options can be specified in the config file;
    /// command-line options will override options in file where there's a conflict. 
    #[structopt(short = "f", long = "file")]
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

         let artnet_address = match cli_config.artnet_address {
            None => match file_config.artnet_address {
                None => return Err(anyhow!("No Art-Net listening address specified.")),
                Some(addr) => addr,
            },
            Some(addr) => addr,
        };

        let kinet_address = match cli_config.kinet_address {
            None => match file_config.kinet_address {
                None => return Err(anyhow!("No KiNET output address specified.")),
                Some(addr) => addr,
            },
            Some(addr) => addr,
        };

        let mut pds_addresses = vec!();
        if cli_config.pds_addresses.is_some() {
            pds_addresses.extend(cli_config.pds_addresses.unwrap());
        }
        if file_config.pds_addresses.is_some() {
            pds_addresses.extend(file_config.pds_addresses.unwrap());
        }
        if pds_addresses.len() == 0 {
            return Err(anyhow!("No KiNET destinations specified."));
        }

        pds_addresses.sort_unstable();
        pds_addresses.dedup();

        let default_verbosity: i8 = 2;
        let verbosity = default_verbosity 
            + cli_config.verbose - cli_config.quiet
            + file_config.verbose - file_config.quiet;
      
        let config = Configuration {
            artnet_address: artnet_address,
            kinet_address: kinet_address,
            pds_addresses: pds_addresses,
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


