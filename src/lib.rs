use std::error::Error;
use std::path::{Path, PathBuf};
use std::{env, fs, io};

use indicatif::ProgressBar;

pub struct Config {
    pub src_dir: PathBuf,
    pub des_dir: PathBuf,
    pub max_depth: usize,
    pub enable_case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next(); // skip program name

        let src_dir = args.next().expect("Source directory is missing");
        let des_dir = args.next().expect("Destination directory is missing");
        let max_depth = args.next().unwrap_or_else(|| "255".to_string());
        let enable_case_sensitive = args.next().unwrap_or_else(|| "false".to_string());

        let src_dir = PathBuf::from(src_dir);
        let des_dir = PathBuf::from(des_dir);
        let max_depth: usize = max_depth.trim().parse().unwrap();
        let enable_case_sensitive: bool = enable_case_sensitive.trim().parse().unwrap();

        if !src_dir.exists() {
            Err("<src_dir> does not exits!!")
        } else if !des_dir.exists() {
            Err("<des_dir> does not exits!!")
        } else {
            Ok(Config {
                src_dir,
                des_dir,
                max_depth,
                enable_case_sensitive,
            })
        }
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let src_files = get_files(config.src_dir, config.max_depth)?;
    let des_files = get_files(config.des_dir, config.max_depth)?;

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
    let pb = ProgressBar::new(src_files.len() as u64);

    for src_file in src_files {
        for des_file in des_files {
            if src_file.file_name() == des_file.file_name() {
                fs::copy(src_file, des_file)?;
            }
        }
        pb.inc(1);
    }
    pb.finish();

    Ok(())
}

pub fn replace_files_case_insensitive(
    src_files: &Vec<PathBuf>,
    des_files: &Vec<PathBuf>,
) -> Result<(), Box<dyn Error>> {
    let pb = ProgressBar::new(src_files.len() as u64);

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
        pb.inc(1);
    }
    pb.finish();

    Ok(())
}

fn is_hidden(dir: &Path) -> bool {
    dir.file_name()
        .unwrap()
        .to_str()
        .map(|s| s.starts_with('.'))
        .unwrap_or(false)
}

fn _store_dirs_and_files(files: &mut Vec<PathBuf>, dirs: &mut Vec<PathBuf>) -> io::Result<()> {
    // create a new copy and empty the dirs vector
    let mut _dirs: Vec<PathBuf> = dirs.drain(..).collect();
    // remove hidden files
    _dirs.retain(|dir| !is_hidden(dir));

    for dir in &_dirs {
        let paths = fs::read_dir(dir)?;
        for path_result in paths {
            let full_path = path_result?.path();
            if full_path.is_dir() {
                dirs.push(full_path)
            } else {
                files.push(full_path)
            }
        }
    }

    Ok(())
}

pub fn get_files(path: PathBuf, max_depth: usize) -> io::Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    let mut dirs = vec![path];

    for _ in 0..max_depth {
        _store_dirs_and_files(&mut files, &mut dirs)?;
    }

    Ok(files)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_files() -> Result<(), Box<dyn Error>> {
        let src_dir = PathBuf::from(r"./test/src");
        let des_dir = PathBuf::from(r"./test/des");

        let mut src_files = get_files(src_dir, 2)?;
        let mut des_files = get_files(des_dir, 2)?;
        src_files.sort();
        des_files.sort();

        assert_eq!(
            src_files,
            [
                PathBuf::from(r"./test/src/a.txt"),
                PathBuf::from(r"./test/src/b.txt"),
                PathBuf::from(r"./test/src/c.txt"),
                PathBuf::from(r"./test/src/d.txt")
            ]
        );

        assert_eq!(
            des_files,
            [
                PathBuf::from(r"./test/des/A/A.txt"),
                PathBuf::from(r"./test/des/b.txt"),
                PathBuf::from(r"./test/des/c.txt"),
                PathBuf::from(r"./test/des/d.txt")
            ]
        );

        Ok(())
    }

    #[test]
    fn test_max_depth() -> Result<(), Box<dyn Error>> {
        let des_dir = PathBuf::from(r"./test/des");

        let mut des_files = get_files(des_dir, 1)?;
        des_files.sort();

        assert_eq!(
            des_files,
            [
                PathBuf::from(r"./test/des/b.txt"),
                PathBuf::from(r"./test/des/c.txt"),
                PathBuf::from(r"./test/des/d.txt")
            ]
        );

        Ok(())
    }

    #[test]
    fn test_replace_files() -> Result<(), Box<dyn Error>> {
        let src_dir = PathBuf::from(r"./test/src");
        let des_dir = PathBuf::from(r"./test/des");

        let mut src_files = get_files(src_dir, 2)?;
        let mut des_files = get_files(des_dir, 2)?;
        src_files.sort();
        des_files.sort();
        // remove 1st 2 files
        src_files.drain(0..2);
        des_files.drain(0..2);

        replace_files(&src_files, &des_files)?;
        let des_c_content = fs::read_to_string(&des_files[0])?;
        let des_d_content = fs::read_to_string(&des_files[1])?;

        assert_eq!(des_c_content, "src/c.txt".to_string());
        assert_eq!(des_d_content, "src/d.txt".to_string());

        Ok(())
    }

    #[test]
    fn test_replace_files_case_insensitive() -> Result<(), Box<dyn Error>> {
        let src_dir = PathBuf::from(r"./test/src");
        let des_dir = PathBuf::from(r"./test/des");

        let mut src_files = get_files(src_dir, 2)?;
        let mut des_files = get_files(des_dir, 2)?;
        src_files.sort();
        des_files.sort();
        // remove last 2 files
        src_files.drain(2..);
        des_files.drain(2..);

        replace_files_case_insensitive(&src_files, &des_files)?;
        let des_a_content = fs::read_to_string(&des_files[0])?;
        let des_b_content = fs::read_to_string(&des_files[1])?;

        assert_eq!(des_a_content, "src/a.txt".to_string());
        assert_eq!(des_b_content, "src/b.txt".to_string());

        Ok(())
    }
}
