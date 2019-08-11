use rusoto_core::Region;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::default::Default;
use std::fs;
use std::io::{stdin, stdout, Error, Write};

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

pub fn request_string(message: &str) -> Result<String, Error> {
    print!("{}", message);
    stdout().flush()?;

    let mut filepath = String::new();
    stdin()
        .read_line(&mut filepath)
        .map(|_| filepath.trim().to_string())
}

pub fn request_ssh_key_path(region: &Region) -> Result<String, Error> {
    let config = Config::load();
    if let Some(path) = config.ssh_key_path.get(region.name()) {
        return Ok(String::from(path));
    }

    request_string("insert ssh key filepath: ")
}
