use std::path::PathBuf;
use std::{fs, io};

use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Source files(s) or a directory.
    src_path: PathBuf,
    /// Destination files(s) or a directory.
    des_path: PathBuf,
}

pub fn run() -> Result<()> {
    let args = Args::parse();
    let src_files = get_files(args.src_path)?;
    let des_files = get_files(args.des_path)?;

    replace_files(&src_files, &des_files)?;

    Ok(())
}

fn get_files(path: PathBuf) -> io::Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    let mut dirs = Vec::new();

    if path.is_dir() {
        dirs.push(path);
    } else {
        files.push(path);
    }

    while let Some(dir) = dirs.pop() {
        for path_result in fs::read_dir(dir)? {
            let full_path = path_result?.path();
            if full_path.is_dir() {
                dirs.push(full_path);
            } else {
                files.push(full_path);
            }
        }
    }

    Ok(files)
}

fn replace_files(src_files: &[PathBuf], des_files: &[PathBuf]) -> Result<()> {
    let pb = ProgressBar::new(src_files.len() as u64);
    pb.set_style(ProgressStyle::with_template("|{bar:40.cyan/blue}| [{elapsed_precise}]").unwrap());

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_files() -> Result<()> {
        let src_dir = PathBuf::from(r"./test/src");
        let des_dir = PathBuf::from(r"./test/des");

        let mut src_files = get_files(src_dir)?;
        let mut des_files = get_files(des_dir)?;
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
    fn test_replace_files() -> Result<()> {
        let src_dir = PathBuf::from(r"./test/src");
        let des_dir = PathBuf::from(r"./test/des");

        let mut src_files = get_files(src_dir)?;
        let mut des_files = get_files(des_dir)?;
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
}
