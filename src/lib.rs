use std::error::Error;
use std::path::{Path, PathBuf};
use std::{fs, io};

use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Path to source directory
    src_dir: PathBuf,
    /// Path to destinatin directory
    des_dir: PathBuf,
    /// Enabling case sensitivity for file names while replacing
    #[arg(short = 'c', long)]
    enable_case_sensitive: bool,
    /// Enabling hidden directories for replacing files
    #[arg(short = 'h', long)]
    enable_hidden_dirs: bool,
    /// maximum allowed depth to recurse through source directory
    #[arg(short = 'd', long, default_value_t = 255)]
    max_depth: usize,
    /// list of file paths to ignore
    #[arg(short = 'i', long)]
    ignore_paths: Vec<PathBuf>,
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();
    let src_files = get_files(
        args.src_dir,
        args.max_depth,
        args.enable_hidden_dirs,
        &args.ignore_paths,
    )?;
    let des_files = get_files(
        args.des_dir,
        args.max_depth,
        args.enable_hidden_dirs,
        &args.ignore_paths,
    )?;

    if args.enable_case_sensitive {
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
    pb.set_style(
        ProgressStyle::with_template("[{wide_bar:.cyan/blue}] [{elapsed_precise}]").unwrap(),
    );

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
    pb.set_style(
        ProgressStyle::with_template("[{wide_bar:.cyan/blue}] [{elapsed_precise}]").unwrap(),
    );

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
        .unwrap_or_default()
        .to_str()
        .map(|s| s.starts_with('.'))
        .unwrap_or(false)
}

fn _store_dirs_and_files(
    files: &mut Vec<PathBuf>,
    dirs: &mut Vec<PathBuf>,
    enable_hidden_dirs: bool,
    ignore_paths: &[PathBuf],
) -> io::Result<()> {
    // create a new copy and empty the dirs vector
    let mut _dirs: Vec<PathBuf> = dirs.drain(..).collect();
    // remove hidden files if flag is enabled
    if !enable_hidden_dirs {
        _dirs.retain(|dir| !is_hidden(dir));
    }

    for dir in &_dirs {
        let paths = fs::read_dir(dir)?;
        for path_result in paths {
            let full_path = path_result?.path();
            if !(ignore_paths.contains(&full_path)) {
                if full_path.is_dir() {
                    dirs.push(full_path)
                } else {
                    files.push(full_path)
                }
            }
        }
    }

    Ok(())
}

pub fn get_files(
    path: PathBuf,
    max_depth: usize,
    enable_hidden_dirs: bool,
    ignore_paths: &[PathBuf],
) -> io::Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    let mut dirs = vec![path];

    for _ in 0..max_depth {
        _store_dirs_and_files(&mut files, &mut dirs, enable_hidden_dirs, ignore_paths)?;
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

        let mut src_files = get_files(src_dir, 2, true, &[])?;
        let mut des_files = get_files(des_dir, 2, true, &[])?;
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

        let mut des_files = get_files(des_dir, 1, true, &[])?;
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

        let mut src_files = get_files(src_dir, 2, true, &[])?;
        let mut des_files = get_files(des_dir, 2, true, &[])?;
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

        let mut src_files = get_files(src_dir, 2, true, &[])?;
        let mut des_files = get_files(des_dir, 2, true, &[])?;
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
