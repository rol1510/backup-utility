use std::fs;
use std::path::PathBuf;

use serde_derive::Deserialize;
use wax::{CandidatePath, Glob, Pattern};

pub const CONFIG_FILE_PATH: &str = "./config.toml";

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

pub fn get_all_files_filtered(unit: &Unit) -> Vec<PathBuf> {
    let files = get_all_files(&unit.base);

    let files = match &unit.exclude {
        None => files,
        Some(patterns) => {
            let filters = patterns
                .into_iter()
                .map(|item| {
                    return Glob::new(&item).unwrap();
                })
                .collect();

            files
                .into_iter()
                .filter_map(|path| {
                    if is_match(&path, &filters) {
                        None
                    } else {
                        Some(path)
                    }
                })
                .collect()
        }
    };

    return files;
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

pub fn read_config() -> Config {
    let config: Config = toml::from_str(&read_file(CONFIG_FILE_PATH)).unwrap();
    return config;
}

fn read_file(path: &str) -> String {
    fs::read_to_string(path).expect("Could not read file")
}
