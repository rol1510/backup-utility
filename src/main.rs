use std::ffi::OsString;
use std::fs;
use std::path::PathBuf;

use clap::Command;
use serde_derive::Deserialize;

const CONFIG_FILE_PATH: &str = "./config.toml";

#[derive(Deserialize, Debug)]
struct Config {
    units: Vec<Unit>,
}

#[derive(Deserialize, Debug)]
struct Unit {
    base: PathBuf,
    exclude: Option<Vec<String>>,
}

fn cli() -> Command<'static> {
    Command::new("git")
        .about("A fictional versioning CLI")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .subcommand(Command::new("show").about("show the config file"))
        .subcommand(Command::new("preview").about("show the config file"))
}

fn show() {
    println!("showing file {CONFIG_FILE_PATH}");

    let config = read_config();

    dbg!(config);
}

fn preview() {
    let config = read_config();

    config.units.into_iter().for_each(|item: Unit| {
        println!("\nUnit {:?}", item.base);
        let files = get_all_files(item.base);
        println!("{:#?}", files);
    });
}

fn get_all_files(path: PathBuf) -> Vec<PathBuf> {
    let x = path.read_dir().expect("a");

    let y = x.flat_map(|item| {
        let p = item.unwrap().path();
        let res: Vec<PathBuf> = if p.is_dir() {
            get_all_files(p)
        } else {
            let mut v = Vec::new();
            v.push(p);
            return v;
        };
        res
    });

    return y.collect();
}

fn read_config() -> Config {
    let config: Config = toml::from_str(&read_file(CONFIG_FILE_PATH)).unwrap();
    return config;
}

fn read_file(path: &str) -> String {
    fs::read_to_string(path).expect("Could not read file")
}

fn main() {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("show", _sub_matches)) => {
            show();
        }
        Some(("preview", _sub_matches)) => {
            preview();
        }
        Some((ext, sub_matches)) => {
            let args = sub_matches
                .get_many::<OsString>("")
                .into_iter()
                .flatten()
                .collect::<Vec<_>>();
            println!("Calling out to {:?} with {:?}", ext, args);
        }
        _ => unreachable!(), // If all subcommands are defined above, anything else is unreachabe!()
    }
}
