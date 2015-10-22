use std::env;

fn main() {
    let the_args : Vec<_> = env::args().collect();

    if the_args.len() != 3 || the_args.len() != 4 {
        println!("Usage: uploadutil <action: upl/dl> <local file> <remote folder>");
    }
}
