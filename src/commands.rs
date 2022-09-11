use std::fs;
use std::path::PathBuf;

use colored::*;

use crate::lib::*;

pub fn show() {
    println!("showing file {}", get_config_file_location().cyan());

    let config = read_config();

    dbg!(config);
}

pub fn preview() {
    let config = read_config();

    config.units.into_iter().for_each(|item: Unit| {
        println!("\nUnit {}", format!("{:?}", item.base).cyan());
        let files = get_annotated_files(&item);

        let mut counter = (0, 0);
        files.iter().for_each(|file| {
            if let FileAnnotation::Nothing(_) = file {
                counter.0 += 1;
            } else if let FileAnnotation::Exclude(_) = file {
                counter.1 += 1;
            }
        });

        println!("{:#?}", files);
        println!(
            "Included: {} files\nExcluded: {} files",
            counter.0.to_string().cyan().bold(),
            counter.1.to_string().cyan().bold()
        );
    });
}

pub fn copy(dest: &PathBuf) {
    let config = read_config();

    config.units.into_iter().for_each(|unit: Unit| {
        println!("\nUnit {:?}", &unit.base);
        get_all_files_filtered(&unit)
            .into_iter()
            .map(|path| {
                match rebase_path_and_insert(&path, &unit.base, dest, &unit.output_dir_name) {
                    Some(new_path) => (path, new_path),
                    None => panic!("could not rebase path!"),
                }
            })
            .for_each(|(from_path, to_path)| {
                print!("  copy {:?} -> {:?}", from_path, to_path);
                fs::create_dir_all(to_path.parent().unwrap()).expect("Could not create directory");
                let bytes_copied = fs::copy(&from_path, &to_path).expect("Could not copy file");
                println!(" \tdone ({bytes_copied} Bytes)");
            });
    });
}

pub fn link(dest: &PathBuf) {
    let config = read_config();

    config.units.into_iter().for_each(|unit: Unit| {
        println!("\nUnit {:?}", &unit.base);
        get_all_files_filtered(&unit)
            .into_iter()
            .map(|path| {
                match rebase_path_and_insert(&path, &unit.base, dest, &unit.output_dir_name) {
                    Some(new_path) => (path, new_path),
                    None => panic!("could not rebase path!"),
                }
            })
            .for_each(|(from_path, to_path)| {
                print!("  link {:?} -> {:?}", from_path, to_path);
                fs::create_dir_all(to_path.parent().unwrap()).expect("Could not create directory");
                fs::hard_link(&from_path, &to_path).expect("Could not copy file");
                println!(" \tdone");
            });
    });
}
