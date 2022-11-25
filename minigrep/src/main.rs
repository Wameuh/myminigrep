use std::env;
use std::process;
use minigrep::Args;


fn main() {
    let args: Vec<String> = env::args().collect();
    let arguments = Args::build(&args).unwrap_or_else(|_| {
        eprintln!("Error in the number of arguments provided");
        process::exit(1);
    });
    
    minigrep::run(arguments).unwrap_or_else(|err| {
        eprintln!("Application error. Error: {}", err);
        process::exit(1);
    });
}
