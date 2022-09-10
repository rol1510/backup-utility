use std::ffi::OsString;
use std::path::PathBuf;

use clap::Command;
use colored::control;

use commands::{copy, preview, show};

mod commands;
mod lib;

fn cli() -> Command<'static> {
    Command::new("backup-utility")
        .about("Make the computer collect your important files for backups.")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .subcommand(Command::new("show").about("show the config file"))
        .subcommand(Command::new("preview").about("preview what files will be included"))
        .subcommand(Command::new("copy").about("do something"))
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
        Some(("copy", _sub_matches)) => {
            copy(&PathBuf::from("R:/tmp/"));
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
