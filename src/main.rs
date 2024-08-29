use minigrep::Config;
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    // let config = match Config::build(&args) {
    //     Ok(config) => config,
    //     Err(msg) => {
    //         println!("{msg}");
    //         process::exit(1);
    //     }
    // };

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("{err}");
        process::exit(1);
    });

    println!("Searching for {:?} in {:?}", config.query, config.file_path);

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
