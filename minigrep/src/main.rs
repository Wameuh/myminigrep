use std::env;
use std::fs;

struct Args<'a> {
    query: &'a String,
    file_path: &'a String
}

#[derive(Debug)]
enum MiniGrepError {
    ArgsError,
    FileError,
}


fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(&args);
    let arguments: Args;

    match get_args(&args) {
        Ok(a) => arguments = a,
        Err(_) => {println!("Error in the arguments provided"); return;}
    }

    println!("Searching for {}", arguments.query);
    println!("In file {}", arguments.file_path);
    
    let contents: String;
    match get_file_content(arguments.file_path) {
        Ok(c) => contents = c,
        Err(_) => {println!("Error while reading the file: {}", arguments.file_path); return;}
    }
    println!("With text:\n{contents}");
}

fn get_args(args: &Vec<String>) -> Result<Args, MiniGrepError> {
    match args.len() {
        3 => Ok(Args {query: &args[1], file_path: &args[2]}),
        _ => Err(MiniGrepError::ArgsError)
    }
}

fn get_file_content(filepath: &String) -> Result<String, MiniGrepError> {
    match fs::read_to_string(filepath) {
        Ok(content) => Ok(content),
        _ => Err(MiniGrepError::FileError)
    }
}