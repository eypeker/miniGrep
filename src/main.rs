#![allow(warnings)]
use std::env;
use std::process;
use minigrep::Config;
fn main() {
    let cfg = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(cfg){
        eprintln!("{}",e);
        process::exit(1);
    }

}

