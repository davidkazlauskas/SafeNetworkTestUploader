#![feature(plugin)]
#![plugin(regex_macros)]

extern crate safe_core;
//extern crate safe_nfs;
//extern crate safe_dns;
extern crate sodiumoxide;
//extern crate routing;
extern crate regex;

use std::env;
use std::io;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::prelude::*;
use std::fs::File;
use safe_core::core::client::Client;
use safe_core::nfs::helper::directory_helper::DirectoryHelper;
use safe_core::nfs::helper::file_helper::FileHelper;
use safe_core::nfs::helper::writer::Writer;
use safe_core::dns::dns_operations::DnsOperations;

fn login(register: bool) -> Client {
    let mut keyword = String::new();
    let mut pin = String::new();
    let mut password = String::new();

    // TEST_SAFENETWORK_LOGIN_PATH file format
    // is three lines, keyword pin and pass.
    //
    // Example:
    //
    // -------
    // test\n
    // 1234\n
    // test\n
    // -------
    // (no dashes)
    //
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

    if !register {
        let res = Client::log_in(keyword.clone(),pin.clone(),password.clone());
        match res {
            Ok(login) => return login,
            Err(err) => {
                println!("Account doesn't exist, will be created...");
                return Client::create_account(
                    keyword.clone(),pin.clone(),password.clone()).unwrap();
            }
        };
    } else {
        return Client::create_account(
            keyword.clone(),pin.clone(),password.clone()).unwrap();
    }
}

fn login_anon() -> Client {
    Client::create_unregistered_client().unwrap()
}

// copy/paste
pub fn path_tokeniser(the_path: String) -> Vec<String> {
    the_path.split("/").filter(|a| !a.is_empty()).map(|a| a.to_string()).collect()
}

pub fn get_directory_key(tokens: &Vec<String>) -> String {
    let tokensize = tokens.len() - 1;
    let mut res = String::new();
    for it in tokens.iter().take(tokensize) {
        res.push_str(&it);
        res.push_str("/");
    }
    res.pop();

    res
}

fn upload_routine(client: std::sync::Arc< std::sync::Mutex< Client > >,local_path: String,remote_path: String) {
    let mut cont : Vec<u8> = Vec::with_capacity(1024 * 1024);

    match File::open(&local_path) {
        Ok(mut file) => file.read_to_end(&mut cont),
        Err(err) => panic!("Cannot open local file."),
    };
}

fn download_routine(client: std::sync::Arc< std::sync::Mutex< Client > >,local_path: String,remote_path: String) {
}

fn download_routine_pub_dns(
    client: std::sync::Arc< std::sync::Mutex< Client > >,
    local_path: String,
    remote_path: String)
{
    let trimmed = remote_path.trim();
    let namergx = regex!(r"^([a-zA-Z0-9_-]+).([a-zA-Z0-9_.-]+)/([a-zA-Z0-9_./]+)$");

    for i in namergx.captures_iter(trimmed) {
        let service = i.at(1).unwrap().to_string();
        let name = i.at(2).unwrap().to_string();
        let file = i.at(3).unwrap().to_string();

        println!("Ze stuff:|{}|{}|{}|",service,name,file);
        return;
    }

    panic!("Should never be reached!");
}

fn reg_dns_routine(client: std::sync::Arc< std::sync::Mutex< Client > >,domain: String) {
    // REG DNS NAME FIRST
    let dirhelper = DirectoryHelper::new(client.clone());
    let dnsops = DnsOperations::new(client.clone()).unwrap();
    let trdomain = domain.trim();

    let mut rootdir = match dirhelper.get_user_root_directory_listing() {
        Ok(val) => val,
        Err(err) => panic!("Could not retrieve user root directory: {:?}",err),
    };

    let service_home_dir = format!("www_{}_home_dir",trdomain);
    let the_dir = match dirhelper.create(service_home_dir,
                                          safe_core::nfs::UNVERSIONED_DIRECTORY_LISTING_TAG,
                                          vec![],
                                          false,
                                          safe_core::nfs::AccessLevel::Public,
                                          Some(&mut rootdir))
    {
        Ok((dir,_)) => dir,
        Err(err) => panic!("Cannot create directory: {:?}",err),
    };

    let dir_key = the_dir.get_key();

    let secret_sign_key =
        match client.lock().unwrap().get_secret_signing_key() {
            Ok(key) => key.clone(),
            Err(err) => panic!("Could not retrieve secret signing key: {:?}",err),
        };

    let pub_client_key =
        match client.lock().unwrap().get_public_signing_key() {
            Ok(res) => res.clone(),
            Err(err) => panic!("Could not retrieve public client key: {:?}",err),
        };

    let (domain_pk,domain_sk) = sodiumoxide::crypto::box_::gen_keypair();
    let owners = vec![pub_client_key];

    let dns_struct_data =
        match dnsops.register_dns(
            trdomain.to_string(),
            &domain_pk,
            &domain_sk,
            &vec![],
            owners,
            &secret_sign_key,
            None)
        {
            Ok(res) => res,
            Err(err) => panic!("Could not make registration data: {:?}",err),
        };
}

// copied and refactored from official
fn test_routine(client: std::sync::Arc< std::sync::Mutex< Client > >) {
}

fn create_sub_directory(client: std::sync::Arc< std::sync::Mutex< Client > >,path: String) {
}

fn print_usage() {
    println!("Usage (upload): uploadutil upl <local file> <remote folder>");
    println!("Usage (download): uploadutil dl <remote file> <local path>");
    println!("Usage (reg www domain): uploadutil reg <domain>");
    println!("Usage (mkdir): uploadutil mkdir <remote path>");
    println!("Usage (test): uploadutil test");
    println!("Usage (register user): uploadutil regu");
}

fn main() {
    let the_args : Vec<_> = env::args().collect();

    if     the_args.len() != 2
        && the_args.len() != 3
        && the_args.len() != 4 {
        print_usage();
        return;
    }

    let command = the_args[1].clone();
    if command == "upl" {
        println!("Logging in...");
        let login = login(false);
        println!("Logged in");
        let login_arc = std::sync::Arc::new( std::sync::Mutex::new(login) );

        println!("Uploading...");
        assert!( the_args.len() == 4, "Upload routine expects three arguments." );
        upload_routine(login_arc.clone(),the_args[2].clone(),the_args[3].clone());
        println!("Done!");
        return;
    } else if command == "reg" {
        println!("Logging in...");
        let login = login(false);
        println!("Logged in");
        let login_arc = std::sync::Arc::new( std::sync::Mutex::new(login) );

        reg_dns_routine(login_arc.clone(),the_args[2].clone());
    } else if command == "dl" {
        println!("Logging in (anonymous)...");
        let login = login_anon();
        println!("Logged in");
        let login_arc = std::sync::Arc::new( std::sync::Mutex::new(login) );
        println!("Downloading...");
        assert!( the_args.len() == 4, "Download routine expects three arguments." );
        download_routine_pub_dns(login_arc.clone(),the_args[3].clone(),the_args[2].clone());
        println!("Done!");
    } else if command == "test" {
        println!("Logging in...");
        let login = login(false);
        println!("Logged in");
        let login_arc = std::sync::Arc::new( std::sync::Mutex::new(login) );

        println!("Testing...");
        assert!( the_args.len() == 2, "Download routine expects three arguments." );
        test_routine(login_arc.clone());
        println!("Done!");
    } else if command == "regu" {
        println!("Registering user...");
        login(true);
        println!("Registered!");
    }
}
