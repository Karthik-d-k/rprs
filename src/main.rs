use std::{env, process};
use rprs::Config;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        eprintln!("Usage: <rprs> <src_dir> <des_dir>");

        process::exit(1);
    });

    if let Err(e) = rprs::run(config) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}
