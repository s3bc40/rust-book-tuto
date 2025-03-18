use std::{env, fs};

fn main() {
    // Reading args
    let args: Vec<String> = env::args().collect();
    // dbg!(args);

    // Saving args
    let config = Config::new(&args);

    println!("Searchin for `{}`", config.query);
    println!("In file `{}`", config.file_path);

    // Reading a file
    let content =
        fs::read_to_string(config.file_path).expect("Should have been able to read the file");
    println!("With text:\n```\n{}\n```", content);
}

// Grouping configuration values
struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        // Refactored code
        // Error handling
        if args.len() < 3 {
            panic!("Not enough arguments: 2 expected")
        }
        // @dev Many Rustaceans avoid using clone to fix ownership problems because of its runtime cost
        // Gain simplicity other perf (fine here)
        let query = args[1].clone();
        let file_path = args.get(2).expect("No file path in args").clone();

        Config { query, file_path }
    }
}
