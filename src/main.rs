use std::process;

fn main() {
    if let Err(e) = rprs::run() {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}
