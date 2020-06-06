use structopt::StructOpt;
use serde::Deserialize;
use log::Level;
use anyhow::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

#[derive(Debug, StructOpt, Deserialize)]
/// Map Art-Net universes to KiNET PDS endpoints
pub struct Configuration {
    /// The network address to listen on 
    #[structopt(short = "l", long = "listen", default_value = "0.0.0.0")]
    pub artnet_address: String,
    /// The network address to send KiNET from   
    #[structopt(short = "k", long = "kinet", required(true))]
    pub kinet_address: String,
    /// The KiNET PDS addresses to send to
    #[structopt(short = "p", long = "pds", required(true))]
    pub pds_addresses: Vec<String>,
    // Make output more verbose. Add -v for debugging info, add -vv for even more detailed message tracing.
    #[structopt(long, short = "v", parse(from_occurrences))]
    pub verbose: i8,
    // Make output less verbose. Add -q to only show warnings and errors, -qq to only show errors, and -qqq to silence output completely.
    #[structopt(long, short = "q", parse(from_occurrences), conflicts_with = "verbose")]
    pub quiet: i8,
    // File to load configuration from
    #[structopt(short = "f", long = "file", required(false), default_value = "")]
    pub config_file: String,
}

impl Default for Configuration {
    fn default() -> Self {
        Configuration {
            artnet_address: "0.0.0.0".to_owned(),
            kinet_address: "0.0.0.0".to_owned(),
            pds_addresses: vec!(),
            verbose: 1,
            quiet: 0,
            config_file: "".to_owned(),
        }
    }
}

impl Configuration {
    pub fn get_log_level(&self) -> Option<Level> {
        let verbosity: i8 = 2 - self.verbose - self.quiet;
        match verbosity {
            std::i8::MIN..=-1 => None,
            0 => Some(log::Level::Error),
            1 => Some(log::Level::Warn),
            2 => Some(log::Level::Info),
            3 => Some(log::Level::Debug),
            4..=std::i8::MAX => Some(log::Level::Trace),
        }
    }

    pub fn read_from_file<P: AsRef<Path>>(path: P) -> Result<Configuration, Error> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
    
        let cfg = serde_json::from_reader(reader)?;
    
        Ok(cfg)
    }
   
}


