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

    let lines = search(&arguments.query, content.as_str());

    if lines.len() == 0 {
        println!("'{}' not found in the file", arguments.query);
    } else {
        println!("{}", lines.iter().fold(String::new(), |acc, &arg| acc + arg + "\n"));
    }
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut retval: Vec<&'a str>= vec![];
    for line in contents.lines() {
        if line.contains(query) {
            retval.push(line);
        }
    }
    retval
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search_one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn search_no_result() {
        let query = "dact";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!({let a:Vec<&str> = vec![];a}, search(query, contents));

    }

    #[test]
    fn search_multiples_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three duct.";

        assert_eq!(vec!["safe, fast, productive.","Pick three duct."], search(query, contents));

    }
}