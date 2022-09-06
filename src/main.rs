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
    output_dir_name: PathBuf,
    exclude: Option<Vec<String>>,
}

fn cli() -> Command<'static> {
    Command::new("git")
        .about("A fictional versioning CLI")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .subcommand(Command::new("show").about("show the config file"))
        .subcommand(Command::new("preview").about("preview what files will be included"))
        .subcommand(Command::new("copy").about("do something"))
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
        let files = get_all_files(&item.base);
        println!("{:#?}", files);
    });
}

fn copy(dest: &PathBuf) {
    let config = read_config();

    config.units.into_iter().for_each(|unit: Unit| {
        println!("\nUnit {:?}", &unit.base);
        get_all_files(&unit.base)
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

fn rebase_path_and_insert(
    path: &PathBuf,
    old_base: &PathBuf,
    new_base: &PathBuf,
    insert: &PathBuf,
) -> Option<PathBuf> {
    if !path.starts_with(old_base) {
        return None;
    }

    let end = path.strip_prefix(old_base).unwrap();
    return Some(new_base.join(insert.join(end)));
}

fn rebase_path(path: &PathBuf, old_base: &PathBuf, new_base: &PathBuf) -> Option<PathBuf> {
    rebase_path_and_insert(path, old_base, new_base, &PathBuf::from(""))
}

#[test]
fn test_rebase_path() {
    let base = PathBuf::from("./a/b/c/");
    assert_eq!(
        rebase_path(&base, &PathBuf::from("./a/"), &PathBuf::from("./")),
        Some(PathBuf::from("./b/c/"))
    );

    assert_eq!(
        rebase_path(&base, &PathBuf::from("./a"), &PathBuf::from("./")),
        Some(PathBuf::from("./b/c/"))
    );

    assert_eq!(
        rebase_path(&base, &PathBuf::from("./a/"), &PathBuf::from("R:/backup/")),
        Some(PathBuf::from("R:/backup/b/c/"))
    );

    assert_eq!(
        rebase_path(&base, &PathBuf::from("./a/"), &PathBuf::from("R:/backup")),
        Some(PathBuf::from("R:/backup/b/c/"))
    );
}

#[test]
fn test_rebase_path_and_insert() {
    let base = PathBuf::from("./a/b/c/");
    assert_eq!(
        rebase_path_and_insert(
            &base,
            &PathBuf::from("./a/"),
            &PathBuf::from("./"),
            &PathBuf::from("foo")
        ),
        Some(PathBuf::from("./foo/b/c/"))
    );
}

fn get_all_files(path: &PathBuf) -> Vec<PathBuf> {
    let x = path.read_dir().expect("a");

    let y = x.flat_map(|item| {
        let p = item.unwrap().path();
        let res: Vec<PathBuf> = if p.is_dir() {
            get_all_files(&p)
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
