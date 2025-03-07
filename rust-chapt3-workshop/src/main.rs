fn main() {
    println!("Fahrenheit to Celsius converter");
    println!(
        "75 degrees Fahrenheit is equal to {} degrees Celsius",
        fahrenheit_to_celsius(75.0)
    );

    println!("");

    println!("Get nth fibonacci number");
    println!("The 2th in Fibonacci is {}", n_fibonacci_number(2));
    println!("The 7th in Fibonacci is {}", n_fibonacci_number(7));
    println!("The 15th in Fibonacci is {}", n_fibonacci_number(15));

    println!("");

    println!("The 12 Days of Christmas");
    the_12_days_of_christmas();
}

// Convert Fahrenheit to Celsius
fn fahrenheit_to_celsius(f_degree: f64) -> f64 {
    return (f_degree - 32.0) / (9.0 / 5.0);
}

// nth Fibonacci number
fn n_fibonacci_number(n: u64) -> u64 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    }
    // Only take n > 1 to make recursivity
    recursive_fibo(0, 1, 2, n)
}

fn recursive_fibo(fibo_n1: u64, fibo_n2: u64, fibo_idx: u64, target_fibo_idx: u64) -> u64 {
    let current_fibo = fibo_n1 + fibo_n2;
    if fibo_idx > target_fibo_idx {
        return current_fibo;
    }
    return recursive_fibo(current_fibo, fibo_n1, fibo_idx + 1, target_fibo_idx);
}

// The 12 Days of Christmas
fn the_12_days_of_christmas() {
    let days = [
        ("first", ""),
        ("second", "Two turtle doves"),
        ("third", "Three French hens"),
        ("fourth", "Four calling birds"),
        ("fifth", "Five gold rings"),
        ("sixth", "Six geese a-laying"),
        ("seventh", "Seven swans a-swimming"),
        ("eighth", "Eight maids a-milking"),
        ("ninth", "Nine ladies dancing"),
        ("tenth", "Ten lords a-leaping"),
        ("eleventh", "Eleven pipers piping"),
        ("twelfth", "Twelve drummers drumming"),
    ];

    for day in 0..(days.len()) {
        println!("On the {} day of Christmas,", days[day].0);
        println!("my true love sent to me");
        for gift in (0..day + 1).rev() {
            if days[gift].1.len() > 0 {
                println!("{}", days[gift].1);
            }
        }
        if day == (days.len() - 1) {
            println!("And a partridge in a pear tree!");
        } else {
            println!("And a partridge in a pear tree.");
        }
        println!("");
    }
}
