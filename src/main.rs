extern crate safe_core;
extern crate safe_nfs;

use std::env;
use std::io;
use std::io::BufReader;
use std::io::BufWriter;
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
            println!("Account doesn't exist, will be created...");
            return Client::create_account(
                keyword.clone(),pin.clone(),password.clone()).unwrap();
        }
    }
}

// copy/paste
pub fn path_tokeniser(the_path: String) -> Vec<String> {
    the_path.split("/").filter(|a| !a.is_empty()).map(|a| a.to_string()).collect()
}

pub fn get_final_subdirectory(client            : ::std::sync::Arc<::std::sync::Mutex<::safe_core::client::Client>>,
                              tokens            : &Vec<String>,
                              starting_directory: Option<&::safe_nfs::metadata::directory_key::DirectoryKey>) -> ::safe_nfs::directory_listing::DirectoryListing
{
    let dir_helper = ::safe_nfs::helper::directory_helper::DirectoryHelper::new(client);

    let mut current_dir_listing = match starting_directory {
        Some(directory_key) => dir_helper.get(directory_key).unwrap(),
        None => dir_helper.get_user_root_directory_listing().unwrap(),
    };

    for it in tokens.iter() {
        current_dir_listing = {
            let current_dir_metadata = current_dir_listing.get_sub_directories().iter().find(|a| *a.get_name() == *it)
                                                                                            .unwrap();
            dir_helper.get(current_dir_metadata.get_key()).unwrap()
        };
    }

    current_dir_listing
}

fn upload_routine(client: std::sync::Arc< std::sync::Mutex< Client > >,local_path: String,remote_path: String) {
    use safe_nfs::helper::writer::Writer;

    let mut cont : Vec<u8> = Vec::with_capacity(1024 * 1024);
    match File::open(&local_path) {
        Ok(mut file) => file.read_to_end(&mut cont),
        Err(err) => panic!("Cannot open local file."),
    };

    let tokenized = path_tokeniser(remote_path.clone());
    let final_subdir = get_final_subdirectory(client.clone(),&tokenized,None);

    let file_helper = safe_nfs::helper::file_helper::FileHelper::new(client);
    let mut writer = file_helper.create(remote_path,Vec::new(),final_subdir).unwrap();
    writer.write(&cont,0);
}

fn create_sub_directory(client: std::sync::Arc< std::sync::Mutex< Client > >,path: String) {
    let dir_helper = safe_nfs::helper::directory_helper::DirectoryHelper::new(client);
}

fn print_usage() {
    println!("Usage (upload): uploadutil upl <local file> <remote folder>");
    println!("Usage (download): uploadutil dl <remote file> <local path>");
    println!("Usage (mkdir): uploadutil mkdir <remote path>");
}

fn main() {
    let the_args : Vec<_> = env::args().collect();

    if the_args.len() != 3 && the_args.len() != 4 {
        print_usage();
        return;
    }

    println!("Logging in...");
    let login = login();
    println!("Logged in");

    let login_arc = std::sync::Arc::new( std::sync::Mutex::new(login) );

    let command = the_args[1].clone();
    if command == "upl" {
        assert!( the_args.len() == 4, "Upload routine expects three arguments." );
        upload_routine(login_arc.clone(),the_args[2].clone(),the_args[3].clone());
    }
}
