extern crate safe_core;

use std::env;
use safe_core::client::Client;

fn login() -> Result< Client, safe_core::errors::CoreError > {
    Client::log_in("moo".to_string(),"goo".to_string(),"goo".to_string())
}

fn main() {
    let the_args : Vec<_> = env::args().collect();

    if the_args.len() != 3 || the_args.len() != 4 {
        println!("Usage: uploadutil <action: upl/dl> <local file> <remote folder>");
    }
}
