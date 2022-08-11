use rprs::Config;
use std::{env, process};

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        eprintln!("Usage: rprs <src_dir> <des_dir> <max_depth(optional)> <enable_case_sensitive(optional)>");

        process::exit(1);
    });

    if let Err(e) = rprs::run(config) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}
