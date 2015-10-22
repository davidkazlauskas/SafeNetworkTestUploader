extern crate safe_core;

use std::env;
use std::io;
use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;
use safe_core::client::Client;

fn login() -> Client {
    let mut keyword = "";
    let mut pin = "";
    let mut password = "";

    match env::var("TEST_SAFENETWORK_LOGIN_PATH") {
        Ok(val) => {
            let f = match File::open(val) {
                Ok(file) => {
                    let reader = BufReader::new(file);
                    let lines : Vec<_> = reader.lines().collect();
                },
                Err(err) => {
                    panic!("Could not open file");
                },
            };
        },
        Err(err) => {
            panic!("No enviroment variable TEST_SAFENETWORK_LOGIN_PATH");
        }
    }

    Client::log_in("moo".to_string(),"goo".to_string(),"goo".to_string()).unwrap()
}

fn main() {
    let the_args : Vec<_> = env::args().collect();

    if the_args.len() != 3 || the_args.len() != 4 {
        println!("Usage: uploadutil <action: upl/dl> <local file> <remote folder>");
    }

    let login = login();
}
