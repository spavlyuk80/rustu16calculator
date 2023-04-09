use clap::Parser;
use serde::{Deserialize, Serialize};
use serde_json::from_reader;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

/// Starts a tcp server from a config file
#[derive(Parser, Debug)]
pub(crate) struct Cli {
    /// path to config file
    #[arg(short, long, value_name = "FILE")]
    pub config: PathBuf,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct ServerConfig {
    pub(crate) port: u16,
    name: String,
}

impl ServerConfig {
    pub(crate) fn new_from_file_config(fname: PathBuf) -> ServerConfig {
        let file = File::open(fname).expect("Could not find config file");
        let reader = BufReader::new(file);
        let config: ServerConfig = from_reader(reader).expect("Could not parse the config file");
        config
    }
}
