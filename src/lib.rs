#![allow(unused)]

use std::error::Error;
use std::path::{Path, PathBuf};
use std::{env, fs, io};

pub struct Config {
    pub src_dir: PathBuf,
    pub des_dir: PathBuf,
    enable_case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next(); // skip program name

        let src_dir = args.next().expect("Source directory is missing");
        let des_dir = args.next().expect("Destination directory is missing");
        let enable_case_sensitive = args.next().unwrap_or_else(|| "false".to_string());

        let src_dir = PathBuf::from(src_dir);
        let des_dir = PathBuf::from(des_dir);
        let enable_case_sensitive: bool = enable_case_sensitive.trim().parse().unwrap();

        if !src_dir.exists() {
            Err("<src_dir> does not exits!!")
        } else if !des_dir.exists() {
            Err("<des_dir> does not exits!!")
        } else {
            Ok(Config {
                src_dir,
                des_dir,
                enable_case_sensitive,
            })
        }
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let src_files = get_files(config.src_dir)?;
    let des_files = get_files(config.des_dir)?;

    if config.enable_case_sensitive {
        replace_files(&src_files, &des_files)?;
    } else {
        replace_files_case_insensitive(&src_files, &des_files)?;
    }

    Ok(())
}

pub fn replace_files(
    src_files: &Vec<PathBuf>,
    des_files: &Vec<PathBuf>,
) -> Result<(), Box<dyn Error>> {
    for src_file in src_files {
        for des_file in des_files {
            if src_file.file_name() == des_file.file_name() {
                fs::copy(src_file, des_file)?;
            }
        }
    }

    Ok(())
}

pub fn replace_files_case_insensitive(
    src_files: &Vec<PathBuf>,
    des_files: &Vec<PathBuf>,
) -> Result<(), Box<dyn Error>> {
    for src_file in src_files {
        for des_file in des_files {
            if src_file
                .file_name()
                .unwrap()
                .eq_ignore_ascii_case(des_file.file_name().unwrap())
            {
                fs::copy(src_file, des_file)?;
            }
        }
    }

    Ok(())
}

fn _get_files(vec: &mut Vec<PathBuf>, path: PathBuf) -> io::Result<()> {
    if path.is_dir() {
        let paths = fs::read_dir(&path)?;
        for path_result in paths {
            let full_path = path_result?.path();
            _get_files(vec, full_path);
        }
    } else {
        vec.push(path);
    }
    Ok(())
}

pub fn get_files<T: Into<PathBuf>>(path: T) -> io::Result<Vec<PathBuf>> {
    let mut vec = Vec::new();
    let path = path.into();
    _get_files(&mut vec, path);
    Ok(vec)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iterate_pathbuf_vec() -> Result<(), Box<dyn Error>> {
        let src_dir = PathBuf::from(r"./tmp/src");
        let src_files = get_files(src_dir)?;

        assert_eq!(
            src_files,
            [
                PathBuf::from(r"./tmp/src/Bar.txt"),
                PathBuf::from(r"./tmp/src/foo.txt")
            ]
        );

        Ok(())
    }

    #[test]
    fn test_replace_files() -> Result<(), Box<dyn Error>> {
        let src_dir = PathBuf::from(r"./tmp/src");
        let des_dir = PathBuf::from(r"./tmp/des");

        let src_files = get_files(src_dir)?;
        let des_files = get_files(des_dir)?;

        replace_files(&src_files, &des_files)?;
        let des_foo_content = fs::read_to_string(&des_files[1])?;

        assert_eq!(des_foo_content, "src/foo.txt".to_string());

        Ok(())
    }

    #[test]
    fn test_replace_files_case_insensitive() -> Result<(), Box<dyn Error>> {
        let src_dir = PathBuf::from(r"./tmp/src");
        let des_dir = PathBuf::from(r"./tmp/des");

        let src_files = get_files(src_dir)?;
        let des_files = get_files(des_dir)?;

        replace_files_case_insensitive(&src_files, &des_files)?;
        let des_bar_content = fs::read_to_string(&des_files[0])?;

        assert_eq!(des_bar_content, "src/Bar.txt".to_string());

        Ok(())
    }
}
