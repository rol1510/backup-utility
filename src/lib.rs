use std::fmt;
use std::fs;
use std::path::PathBuf;

use colored::*;
use serde_derive::Deserialize;
use wax::{CandidatePath, Glob, Pattern};

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

pub fn rebase_path_and_insert(
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

fn is_match(path: &PathBuf, filters: &Vec<Glob>) -> bool {
    for glob in filters {
        if glob.is_match(CandidatePath::from(path.as_path())) {
            return true;
        }
    }
    return false;
}

pub enum FileAnnotation {
    Nothing(PathBuf),
    Exclude(PathBuf),
}

impl fmt::Debug for FileAnnotation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                FileAnnotation::Nothing(path) => path.to_str().unwrap().green().bold(),
                FileAnnotation::Exclude(path) => path.to_str().unwrap().normal(),
            }
        )
    }
}

pub fn get_annotated_files(unit: &Unit) -> Vec<FileAnnotation> {
    let files = get_all_files(&unit.base);

    let files = match &unit.exclude {
        None => files
            .into_iter()
            .map(|path| FileAnnotation::Nothing(path))
            .collect(),
        Some(patterns) => {
            let filters = patterns
                .into_iter()
                .map(|item| {
                    return Glob::new(&item).unwrap();
                })
                .collect();

            files
                .into_iter()
                .map(|path| {
                    if is_match(&path, &filters) {
                        FileAnnotation::Exclude(path)
                    } else {
                        FileAnnotation::Nothing(path)
                    }
                })
                .collect()
        }
    };

    return files;
}

pub fn get_all_files_filtered(unit: &Unit) -> Vec<PathBuf> {
    let files = get_annotated_files(&unit);

    files
        .into_iter()
        .filter_map(|file| match file {
            FileAnnotation::Nothing(path) => Some(path),
            FileAnnotation::Exclude(_) => None,
        })
        .collect()
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
