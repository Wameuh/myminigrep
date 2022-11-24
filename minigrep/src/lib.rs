use std::fs;
use std::error::Error;

pub struct Args {
    pub query: String,
    pub file_path: String
}

impl Args {
    pub fn build(args: &Vec<String>) -> Result<Args, &'static str> {
        match args.len() {
            3 => Ok(Args {query: args[1].clone(), file_path: args[2].clone()}),
            _ => Err("Error, number of argument provided is incorrect")
        }
    }
}

pub fn run (arguments: Args) -> Result<(), Box<dyn Error>> {
    println!("reading file {}", &arguments.file_path);
    let content = fs::read_to_string(arguments.file_path)?;

    println!("With text:\n{content}");
    Ok(())
}