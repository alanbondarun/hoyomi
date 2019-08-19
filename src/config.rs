use rusoto_core::Region;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::default::Default;
use std::error::Error;
use std::fs;
use std::io::{stdin, stdout, Write};
use std::str::FromStr;

#[derive(Serialize, Deserialize)]
struct Config {
    pub ssh_key_path: HashMap<String, String>,
}

impl Config {
    fn load() -> Config {
        fs::read_to_string(dirs::home_dir().unwrap().join(".hoyomi_config"))
            .map(|content| serde_json::from_str(&content).unwrap())
            .unwrap_or_default()
    }
}

impl Default for Config {
    fn default() -> Config {
        Config {
            ssh_key_path: HashMap::new(),
        }
    }
}

pub fn request_string(message: &str) -> Result<String, std::io::Error> {
    print!("{}", message);
    stdout().flush()?;

    let mut filepath = String::new();
    stdin()
        .read_line(&mut filepath)
        .map(|_| filepath.trim().to_string())
}

pub fn request_region() -> Result<Region, Box<dyn Error>> {
    request_string("insert region: ")
        .map_err(|err| err.into())
        .and_then(|region| Region::from_str(&region).map_err(|err| err.into()))
}

pub fn request_ssh_key_path(region: &Region) -> Result<String, Box<dyn Error>> {
    let config = Config::load();
    if let Some(path) = config.ssh_key_path.get(region.name()) {
        return Ok(String::from(path));
    }

    request_string("insert ssh key filepath: ").map_err(|err| err.into())
}
