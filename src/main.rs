extern crate safe_core;
extern crate safe_ffi;

use std::env;
use std::io;
use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;
use safe_core::client::Client;

fn login() -> Client {
    let mut keyword = String::new();
    let mut pin = String::new();
    let mut password = String::new();

    match env::var("TEST_SAFENETWORK_LOGIN_PATH") {
        Ok(val) => {
            let f = match File::open(val) {
                Ok(file) => {
                    let reader = BufReader::new(file);
                    let lines : Vec<_> = reader.lines().map(|x| x.unwrap()).collect();

                    if lines.len() != 3 {
                        panic!("Wrong amount of lines! Expected keyword, pin, password (3)");
                    }

                    keyword = lines[0].clone();
                    pin = lines[1].clone();
                    password = lines[2].clone();
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

    let res = Client::log_in(keyword.clone(),pin.clone(),password.clone());
    match res {
        Ok(login) => return login,
        Err(err) => {
            return Client::create_account(
                keyword.clone(),pin.clone(),password.clone()).unwrap();
        }
    }
}

fn main() {
    let the_args : Vec<_> = env::args().collect();

    if the_args.len() != 3 || the_args.len() != 4 {
        println!("Usage (upload): uploadutil upl <local file> <remote folder>");
        println!("Usage (download): uploadutil dl <remote file> <local path>");
        println!("Usage (mkdir): uploadutil mkdir <remote path>");
        return;
    }

    let login = login();
}
