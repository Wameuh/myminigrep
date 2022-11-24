use std::env;
use std::process;
use minigrep::Args;


fn main() {
    let args: Vec<String> = env::args().collect();
    let arguments = Args::build(&args).unwrap_or_else(|_| {
        println!("Error in the number of arguments provided");
        process::exit(1);
    });
    
    minigrep::run(arguments).unwrap_or_else(|err| {
        println!("Application error. Error: {}", err);
        process::exit(1);
    });
}
