use std::error::Error;
use std::fs;

// Grouping configuration values
pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        // Refactored code
        // Error handling
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        // @dev Many Rustaceans avoid using clone to fix ownership problems because of its runtime cost
        // Gain simplicity other perf (fine here)
        let query = args[1].clone();
        let file_path = args.get(2).expect("No file path in args").clone();

        Ok(Config { query, file_path })
    }
}

// Extacting logic from main
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // Reading a file
    let contents = fs::read_to_string(config.file_path)?;

    // Using search function in run
    for line in search(&config.query, &contents) {
        println!("{line}");
    }

    // Returning errors from the run function
    Ok(())
}

// @dev Lifetime important to stick to contents -> lifetime of return val
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    // Iterate throught lines with `lines` method
    for line in contents.lines() {
        // Searching each line for query
        if line.contains(query) {
            // Storing matchin lines
            results.push(line);
        }
    }

    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    vec![]
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
