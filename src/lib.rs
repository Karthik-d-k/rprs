#![allow(unused)]

use std::error::Error;
use std::path::{Path, PathBuf};
use std::{env, fs, io};

pub struct Config {
    pub src_dir: PathBuf,
    pub des_dir: PathBuf,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        let src_dir = std::env::args()
            .nth(1)
            .expect("Source directory is missing");
        let des_dir = std::env::args()
            .nth(2)
            .expect("Destination directory is missing");

        let src_dir = PathBuf::from(src_dir);
        let des_dir = PathBuf::from(des_dir);

        if !src_dir.exists() {
            Err("<src_dir> does not exits!!")
        } else if !des_dir.exists() {
            Err("<des_dir> does not exits!!")
        } else {
            Ok(Config { src_dir, des_dir })
        }
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    for entry in list_files(config.src_dir)? {
        println!("{:?}", entry);
    }
    for entry in list_files(config.des_dir)? {
        println!("{:?}", entry);
    }

    Ok(())
}

fn _list_files(vec: &mut Vec<PathBuf>, path: PathBuf) -> io::Result<()> {
    if path.is_dir() {
        let paths = fs::read_dir(&path)?;
        for path_result in paths {
            let full_path = path_result?.path();
            _list_files(vec, full_path);
        }
    } else {
        vec.push(path);
    }
    Ok(())
}

fn list_files<T: Into<PathBuf>>(path: T) -> io::Result<Vec<PathBuf>> {
    let mut vec = Vec::new();
    let path = path.into();
    _list_files(&mut vec, path);
    Ok(vec)
}
