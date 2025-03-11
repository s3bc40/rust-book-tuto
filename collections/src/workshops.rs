use std::collections::HashMap;

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
