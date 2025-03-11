pub mod workshop1 {
    use std::collections::HashMap;

    pub fn solve() {
        println!("\n");

        println!("Workshop 1: Vector of Integers");
        // Sort the list of integers in ascending order
        let list_of_int = vec![100, 31, 42, 8, 99, 1, 23, 10, 6, 23, 77, 23, 1];
        let mut sorted_int = list_of_int.clone();
        sorted_int.sort();
        println!("Original: {list_of_int:?}");
        println!("Sorted: {sorted_int:?}");

        // Get the median of the list of integers
        let index_median = list_of_int.len() / 2;
        let median = sorted_int.get(index_median).unwrap_or(&0);
        println!("Median: {median}");

        // Get the mode
        let mut int_count: HashMap<i32, i32> = HashMap::new();
        let mut mode = 0;
        let mut max_count = 0;

        for number in &sorted_int {
            let count = int_count.entry(*number).or_insert(0);
            *count += 1;
            if *count > max_count {
                max_count = *count;
                mode = *number;
            }
        }
        println!("Mode: {mode} with {max_count} counts");

        println!("\n");
    }
}

pub mod workshop2 {
    pub fn solve() {
        println!("\n");

        println!("Workshop 2: Convert string to Pig Latin");
        let word1 = String::from("apple");
        let word2 = String::from("first");
        let word3 = String::from("hello");
        let word4 = String::from("yellow");

        let pig_latin1 = convert_to_pig_latin(&word1);
        let pig_latin2 = convert_to_pig_latin(&word2);
        let pig_latin3 = convert_to_pig_latin(&word3);
        let pig_latin4 = convert_to_pig_latin(&word4);

        println!("{word1} -> {pig_latin1}");
        println!("{word2} -> {pig_latin2}");
        println!("{word3} -> {pig_latin3}");
        println!("{word4} -> {pig_latin4}");

        println!("\n");
    }

    fn convert_to_pig_latin(word: &String) -> String {
        let vowels = vec!['a', 'e', 'i', 'o', 'u', 'y'];
        let first_char = word.chars().next().unwrap();
        let pig_latin: String;

        if vowels.contains(&first_char) {
            pig_latin = format!("{word}-hay");
        } else {
            pig_latin = format!("{}-{}ay", word[1..].to_string(), first_char);
        }

        pig_latin
    }
}
