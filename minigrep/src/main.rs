use std::process;

use minigrep::Config;


fn main() {
    let config = Config::new(std::env::args()).unwrap_or_else(|e| {
        eprintln!("\
Problem parsing arguments: {}.
Usage: minigrep <query> <filename>", e);
        process::exit(1);
    });
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
