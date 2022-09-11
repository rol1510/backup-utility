use std::fs;
use std::path::PathBuf;

use byte_unit;
use colored::*;

use crate::couter::Counter;
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

pub fn info() {
    macro_rules! output {
        ($name:expr, $a:expr, $b:expr) => {
            println!(
                "{} {} of {}",
                $name,
                $a.to_string().cyan().bold(),
                $b.to_string().normal()
            );
        };
    }
    macro_rules! bytes_with_unit {
        ($number:expr) => {
            byte_unit::Byte::from_bytes($number as u128).get_appropriate_unit(false)
        };
    }

    let config = read_config();

    let mut counter_sum = Counter::<u32>::new();
    let mut size_sum = Counter::<u64>::new();

    config.units.into_iter().for_each(|item: Unit| {
        println!("\nUnit {}", format!("{:?}", item.base).cyan());
        let files = get_annotated_files(&item);

        let mut counter = Counter::<u32>::new();
        let mut size = Counter::<u64>::new();

        files.iter().for_each(|file| {
            if let FileAnnotation::Nothing(path) = file {
                counter.included += 1;
                size.included += fs::metadata(path).unwrap().len();
            } else if let FileAnnotation::Exclude(path) = file {
                counter.excluded += 1;
                size.excluded += fs::metadata(path).unwrap().len();
            }
        });

        output!("Files:", counter.included, counter.sum());

        output!(
            "Size: ",
            bytes_with_unit!(size.included),
            bytes_with_unit!(size.sum())
        );

        counter_sum += counter;
        size_sum += size;
    });

    println!("\nAll Units");
    output!("Files:", counter_sum.included, counter_sum.sum());

    output!(
        "Size: ",
        bytes_with_unit!(size_sum.included),
        bytes_with_unit!(size_sum.sum())
    );
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
