use std::path::PathBuf;

use wax::{CandidatePath, Glob, Pattern};

use super::config::Unit;
use super::file_annotation::FileAnnotation;

fn is_match(path: &PathBuf, filters: &Vec<Glob>) -> bool {
    for glob in filters {
        if glob.is_match(CandidatePath::from(path.as_path())) {
            return true;
        }
    }
    return false;
}

pub fn annotate_files(files: &Vec<PathBuf>, exclude: &Option<Vec<String>>) -> Vec<FileAnnotation> {
    match exclude {
        None => files
            .into_iter()
            .map(|path| FileAnnotation::Nothing(path.clone()))
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
                    if is_match(path, &filters) {
                        FileAnnotation::Exclude(path.clone())
                    } else {
                        FileAnnotation::Nothing(path.clone())
                    }
                })
                .collect()
        }
    }
}

pub fn get_annotated_files(unit: &Unit) -> Vec<FileAnnotation> {
    let files = get_all_files_recursive(&unit.base);
    annotate_files(&files, &unit.exclude)
}

fn filter_files(files: Vec<FileAnnotation>) -> Vec<PathBuf> {
    files
        .into_iter()
        .filter_map(|file| match file {
            FileAnnotation::Nothing(path) => Some(path),
            FileAnnotation::Exclude(_) => None,
        })
        .collect()
}

pub fn get_all_files_filtered(unit: &Unit) -> Vec<PathBuf> {
    let files = get_annotated_files(&unit);
    filter_files(files)
}

fn get_all_files_recursive(path: &PathBuf) -> Vec<PathBuf> {
    let x = path.read_dir().expect("a");

    let y = x.flat_map(|item| {
        let p = item.unwrap().path();
        let res: Vec<PathBuf> = if p.is_dir() {
            get_all_files_recursive(&p)
        } else {
            let mut v = Vec::new();
            v.push(p);
            return v;
        };
        res
    });

    return y.collect();
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

#[cfg(test)]
mod tests {
    use super::super::file_annotation::FileAnnotation::*;
    use super::super::path_util::*;
    use std::path::PathBuf;
    use std::str::FromStr;
    use std::vec::Vec;

    macro_rules! path_vec {
        ($($x:expr),*) => {
            vec![
                $(PathBuf::from_str($x).unwrap(),)*
            ]
        };
    }

    macro_rules! exclude_vec {
        ($($x:expr),*) => {
            vec![
                $($x.to_string(),)*
            ]
        };
    }

    macro_rules! res_vec {
        ($(($type:expr, $x:expr)),*) => {
            vec![
                $(
                    $type(PathBuf::from_str($x).unwrap()),
                )*
            ]
        };
    }

    #[test]
    fn test_rebase_path() {
        let base = PathBuf::from("./a/b/c/");

        assert_eq!(
            rebase_path_and_insert(
                &base,
                &PathBuf::from("./a/"),
                &PathBuf::from("./"),
                &PathBuf::from("")
            ),
            Some(PathBuf::from("./b/c/"))
        );

        assert_eq!(
            rebase_path_and_insert(
                &base,
                &PathBuf::from("./a"),
                &PathBuf::from("./"),
                &PathBuf::from("")
            ),
            Some(PathBuf::from("./b/c/"))
        );

        assert_eq!(
            rebase_path_and_insert(
                &base,
                &PathBuf::from("./a/"),
                &PathBuf::from("R:/backup/"),
                &PathBuf::from("")
            ),
            Some(PathBuf::from("R:/backup/b/c/"))
        );

        assert_eq!(
            rebase_path_and_insert(
                &base,
                &PathBuf::from("./a/"),
                &PathBuf::from("R:/backup"),
                &PathBuf::from("")
            ),
            Some(PathBuf::from("R:/backup/b/c/"))
        );

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

    #[test]
    fn test_annotate_files() {
        let files: Vec<PathBuf> = path_vec![
            "C:/file.txt",
            "C:/a/file-1",
            "C:/a/file-2",
            "C:/a/image.png",
            "C:/a/image.png.bak",
            "C:/a/file.txt",
            "C:/a/file.tmp",
            "C:/b/file.tmp",
            "C:/b/image.png",
            "C:/b/sub/image.png"
        ];

        let nothing = res_vec![
            (Nothing, "C:/file.txt"),
            (Nothing, "C:/a/file-1"),
            (Nothing, "C:/a/file-2"),
            (Nothing, "C:/a/image.png"),
            (Nothing, "C:/a/image.png.bak"),
            (Nothing, "C:/a/file.txt"),
            (Nothing, "C:/a/file.tmp"),
            (Nothing, "C:/b/file.tmp"),
            (Nothing, "C:/b/image.png"),
            (Nothing, "C:/b/sub/image.png")
        ];
        let all = res_vec![
            (Exclude, "C:/file.txt"),
            (Exclude, "C:/a/file-1"),
            (Exclude, "C:/a/file-2"),
            (Exclude, "C:/a/image.png"),
            (Exclude, "C:/a/image.png.bak"),
            (Exclude, "C:/a/file.txt"),
            (Exclude, "C:/a/file.tmp"),
            (Exclude, "C:/b/file.tmp"),
            (Exclude, "C:/b/image.png"),
            (Exclude, "C:/b/sub/image.png")
        ];
        let mix1 = res_vec![
            (Exclude, "C:/file.txt"),
            (Nothing, "C:/a/file-1"),
            (Nothing, "C:/a/file-2"),
            (Nothing, "C:/a/image.png"),
            (Nothing, "C:/a/image.png.bak"),
            (Nothing, "C:/a/file.txt"),
            (Nothing, "C:/a/file.tmp"),
            (Exclude, "C:/b/file.tmp"),
            (Exclude, "C:/b/image.png"),
            (Exclude, "C:/b/sub/image.png")
        ];
        let mix2 = res_vec![
            (Nothing, "C:/file.txt"),
            (Exclude, "C:/a/file-1"),
            (Exclude, "C:/a/file-2"),
            (Exclude, "C:/a/image.png"),
            (Nothing, "C:/a/image.png.bak"),
            (Nothing, "C:/a/file.txt"),
            (Nothing, "C:/a/file.tmp"),
            (Nothing, "C:/b/file.tmp"),
            (Exclude, "C:/b/image.png"),
            (Exclude, "C:/b/sub/image.png")
        ];

        assert_eq!(annotate_files(&files, &None), nothing);
        assert_eq!(annotate_files(&files, &Some(exclude_vec!["**/*"])), all);
        assert_eq!(
            annotate_files(&files, &Some(exclude_vec!["**/b/**/*", "*/file.txt"])),
            mix1
        );
        assert_eq!(
            annotate_files(&files, &Some(exclude_vec!["**/*.png", "**/file-?"])),
            mix2
        );

        // assert!(false);
    }

    #[test]
    fn test_filter_files() {
        let annoteted = res_vec!(
            (Exclude, "a"),
            (Nothing, "b"),
            (Exclude, "c"),
            (Exclude, "d"),
            (Nothing, "e")
        );

        assert_eq!(filter_files(annoteted), path_vec!["b", "e"]);
    }
}
