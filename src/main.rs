use std::ffi::OsString;
use std::path::PathBuf;

use clap::arg;
use clap::Command;
use colored::control;

use commands::{copy, info, link, preview, show};

mod commands;
mod lib;

pub const ABOUT_STRING: &str = "\
Make the computer collect your important files for backups.\n\n\
Use <subcommand> --help to get info on the usage of the subcommand";

fn cli() -> Command<'static> {
    Command::new("backup-utility")
        .about(ABOUT_STRING)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .subcommand(Command::new("show").about("show the config file"))
        .subcommand(Command::new("preview").about("preview what files will be included"))
        .subcommand(Command::new("info").about("shows some info about the units"))
        .subcommand(
            Command::new("copy")
                .about("copy all files into the specified path")
                .arg(arg!(<PATH> "The path to the output directory")),
        )
        .subcommand(
            Command::new("link")
                .about("like copy, but will create hard links")
                .arg(arg!(<PATH> "The path to the output directory")),
        )
}

fn main() {
    // This is needed for the colored terminal output on Windows.
    control::set_virtual_terminal(true).unwrap();

    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("show", _sub_matches)) => {
            show();
        }
        Some(("preview", _sub_matches)) => {
            preview();
        }
        Some(("info", _sub_matches)) => {
            info();
        }
        Some(("copy", _sub_matches)) => {
            let path = _sub_matches.get_one::<String>("PATH").unwrap();
            copy(&PathBuf::from(path));
        }
        Some(("link", _sub_matches)) => {
            let path = _sub_matches.get_one::<String>("PATH").unwrap();
            link(&PathBuf::from(path));
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
