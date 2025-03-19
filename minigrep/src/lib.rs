use std::error::Error;
use std::{env, fs};

// Grouping configuration values
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    // Using the Returned Iterator Directly
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        // Using Iterator Trait Methods Instead of Indexing
        args.next(); // first value in `env::args`` is the name of the program

        // @dev Many Rustaceans avoid using clone to fix ownership problems because of its runtime cost
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Query string not found"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("File path not found"),
        };

        // Bring env variable
        let ignore_case = match args.next() {
            Some(arg) => arg == "1",
            None => env::var("IGNORE_CASE").is_ok(),
        };

        // Old way before iterator
        // env::var("IGNORE_CASE").is_ok()
        // let ignore_case = if args.get(3).is_some() {
        //     args.get(3).unwrap() == "1"
        // } else {
        //     env::var("IGNORE_CASE").is_ok()
        // };

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

// Extacting logic from main
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // Reading a file
    let contents = fs::read_to_string(config.file_path)?;

    // Using search function in run
    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    // Returning errors from the run function
    Ok(())
}

// @dev Lifetime important to stick to contents -> lifetime of return val
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // Old way without iterator
    // Iterate throught lines with `lines` method
    // let mut results = Vec::new();
    // for line in contents.lines() {
    //     // Searching each line for query
    //     if line.contains(query) {
    //         // Storing matchin lines
    //         results.push(line);
    //     }
    // }
    // results

    // With iterator
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();

    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}

// ------------------------------------------------------------------
//                              TESTS
// ------------------------------------------------------------------

// TDD
// 1. Write a test that fails and run it to make sure it fails for the reason you expect.
// 2. Write or modify just enough code to make the new test pass.
// 3. Refactor the code you just added or changed and make sure the tests continue to pass.
// 4. Repeat from step 1!

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        )
    }
}
