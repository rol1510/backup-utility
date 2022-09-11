use std::fmt;
use std::path::PathBuf;

use colored::*;

#[derive(PartialEq)]
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
