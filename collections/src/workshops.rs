pub mod workshop1 {
    /// Workshop 1: Vector of Integers
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
    /// Workshop 2: Convert string to Pig Latin
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
        // Convert a word to Pig Latin
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

pub mod workshop3 {
    /// Workshop 3: Company Department
    use std::collections::HashMap;
    use std::io;

    pub fn solve() {
        println!("\n");

        let mut company_department: HashMap<String, Vec<String>> = HashMap::new();

        // Program loop to handle a company departments
        loop {
            println!("Chose your option:");
            println!("1: Add a person to a department");
            println!("2: Show department list");
            println!("3: Stop the program");

            let mut option = String::new();
            io::stdin().read_line(&mut option).unwrap();

            match option.trim() {
                "1" => add_employee(&mut company_department),
                "2" => show_department_list(&company_department),
                "3" => {
                    println!("Shutting down");
                    break;
                }
                _ => println!("Invalid input"),
            }
        }

        println!("\n");
    }

    fn add_employee(company_department: &mut HashMap<String, Vec<String>>) {
        // Add an employee to a department
        println!("Type in the name of your employee: ");
        let mut employee = String::new();
        io::stdin().read_line(&mut employee).unwrap();
        println!("\nChose a department to place {employee}: ");
        let mut department = String::new();
        io::stdin().read_line(&mut department).unwrap();

        if !company_department.contains_key(&department) {
            company_department.insert(department.clone(), Vec::new());
        }
        let employee_list = company_department.get_mut(&department).unwrap();
        employee_list.push(String::from(employee.trim()));
        println!("Employee {employee} has been added to {department} department\n");
    }

    fn show_department_list(company_department: &HashMap<String, Vec<String>>) -> () {
        // Show the list of employees in a department
        println!("Type in the department: ");
        let mut department = String::new();
        io::stdin().read_line(&mut department).unwrap();

        if !company_department.contains_key(&department) {
            println!("{department} does not exist!");
            return;
        }
        println!("{department} employees:");
        println!("{:#?}", company_department.get(&department).unwrap())
    }
}
