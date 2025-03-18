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
    let content = fs::read_to_string(config.file_path)?;
    println!("With text:\n```\n{}\n```", content);

    // Returning errors from the run function
    Ok(())
}
