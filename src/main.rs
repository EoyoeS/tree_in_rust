use std::{env, process};

use tree::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    println!("{:?}", config);
    if let Err(e) = tree::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
