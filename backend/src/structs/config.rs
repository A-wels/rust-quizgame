use std::fs::{File, OpenOptions};
use std::io::Read;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub question_file: String,
    pub password: String,
}
impl Config {
    pub fn new() -> Config {
        // check if config.yml exists, if not, create it
        let config = match read_config_file("config.yml") {
            Ok(config) => config,
            Err(_) => {
                write_default_config_file("config.yml").unwrap();
                Config {
                    password: String::from("changeme"),
                    question_file: String::from("questions.csv"),
                }
            }
        };
        return config;

    }
}


fn read_config_file(file_path: &str) -> Result<Config, Box<dyn std::error::Error>> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let config: Config = serde_yaml::from_str(&contents)?;

    Ok(config)
}

fn write_default_config_file(file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let config = Config {
        password: String::from("changeme"),
        question_file: String::from("questions.csv"),
    };

    let file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(file_path)?;

    serde_yaml::to_writer(file, &config)?;

    Ok(())
}