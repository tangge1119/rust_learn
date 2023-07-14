use std::{error::Error, fs};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents =
        fs::read_to_string(config.file_path).expect("Should have been able to read the file");

    println!("With text:\n{contents}");

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    vec![]
}

pub struct Config<'a> {
    pub query: &'a str,
    pub file_path: &'a str,
}

impl<'a> Config<'a> {
    pub fn build(args: &'a [String]) -> Result<Config<'a>, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = &args[1];
        let file_path = &args[2];

        Ok(Config { query, file_path })
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
        Rust:
        safe, fast, productive.
        Pick three.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}
