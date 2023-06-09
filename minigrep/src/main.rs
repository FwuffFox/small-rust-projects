use std::{env, process};
use minigrep::{
    run,
    config::Config
};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config: Config = Config::new(&args).unwrap_or_else(|err| {
        println!("Finished with error: {}", err);
        process::exit(1);
    });

    if let Err(err) = run(config) {
        println!("Finished with error: {}", err);
        process::exit(1);
    }
}


