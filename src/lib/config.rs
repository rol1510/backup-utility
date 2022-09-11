use std::fs;
use std::path::PathBuf;

use colored::*;
use serde_derive::Deserialize;

pub const CONFIG_FILE_PATH_ENV_KEY: &str = "BU_CONFIG";

#[derive(Deserialize, Debug)]
pub struct Config {
    pub units: Vec<Unit>,
}

#[derive(Deserialize, Debug)]
pub struct Unit {
    pub base: PathBuf,
    pub output_dir_name: PathBuf,
    pub exclude: Option<Vec<String>>,
}

pub fn get_config_file_location() -> String {
    match std::env::var(CONFIG_FILE_PATH_ENV_KEY) {
        Ok(val) => val,
        Err(e) => {
            println!(
                "{}",
                format!(
                    "Couldn't get the config file location from '{}'",
                    CONFIG_FILE_PATH_ENV_KEY
                )
                .red()
                .bold()
            );
            println!("Because: {e}");
            std::process::exit(1);
        }
    }
}

pub fn read_config() -> Config {
    let contents = read_file(&get_config_file_location());
    toml::from_str(&contents).unwrap()
}

fn read_file(path: &str) -> String {
    fs::read_to_string(path).expect("Could not read file")
}
