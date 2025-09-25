use std::env;
use std::process;
use colored::*;
use tozed_password_rust_tool::run;
use tozed_password_rust_tool::Config;


fn main() {
    let args: Vec<String> = env::args().collect();
    let config: Config = Config::new(&args).unwrap_or_else(|err|{
        println!("Error: {}",err.red());
        process::exit(1);
    });
    println!("IMEI: {}",config.imei);
    println!("MAC: {}",config.mac);

    if let Err(e) = run(&config) {
        println!("Application error: {}",e);
        process::exit(1);        
    }
}
