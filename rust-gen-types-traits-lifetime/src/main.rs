// fn largest_i32(list: &[i32]) -> &i32 {
//     let mut largest = &list[0];

//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }

// fn largest_char(list: &[char]) -> &char {
//     let mut largest = &list[0];

//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }
fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// Struct definitions
struct Point<T, U> {
    x: T,
    y: U,
}

// Enum definit -> see Option and Result

// In method definitions
impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }
}
// With concrete type to scope methods
impl Point<f32, f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
// Generic type different in method
impl<X1, Y1> Point<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

// Lifetime struct
struct ImportantExcerpt<'a> {
    part: &'a str,
}

// Lifetime annot methods
// Checl lifetime elision rule 3 steps
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }

    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {announcement}");
        self.part
    }
}

use aggregator::{NewsArticle, Summary, Tweet};
fn main() {
    // Fn generic
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {result}");

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {result}");

    // Struct generic
    let integer_and_float = Point { x: 5, y: 4.0 };
    let both_float = Point { x: 2.5, y: 5.4 };
    let both_integer = Point { x: 9, y: 6 };

    // Method
    let p = Point { x: 5, y: 10 };
    println!("p.x = {}", p.x());

    // Mixup
    let p1 = Point { x: 5, y: 1.2 };
    let p2 = Point { x: "Hi", y: 'v' };

    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

    // Implementing trait aggregator example
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    // With default behavior
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    println!("New article available! {}", article.summarize());

    // Trait as Parameters
    pub fn notify(item: &impl Summary) {
        println!("Breaking news! {}", item.summarize());
    }
    // Trait bound syntax
    // Equivalent to previous impl
    // pub fn notify<T: Summary>(item: &T) {
    //     println!("Breaking news! {}", item.summarize());
    // }

    // Appropriate if we want item1 and 2 param be different
    // pub fn notify(item1: &impl Summary, item2: &impl Summary) {

    // If we want to force the type, trait bound syntax is better
    // pub fn notify<T: Summary>(item1: &T, item2: &T) {

    // Multiple trait bounds with + syntax
    // pub fn notify(item: &(impl Summary + Display)) {
    // pub fn notify<T: Summary + Display>(item: &T) {

    notify(&article);

    // Cleared trait bounds with where clause
    // From this
    // fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
    // To this
    // fn some_function<T, U>(t: &T, u: &U) -> i32
    // where
    //     T: Display + Clone,
    //     U: Clone + Debug,
    // {

    // Returning types that implements traits
    fn returns_summarizable() -> impl Summary {
        Tweet {
            username: String::from("horse_ebooks"),
            content: String::from("of course, as you probably already know, people"),
            reply: false,
            retweet: false,
        }
    }

    // This would not work because impl Trait allows to retunr one type
    // fn returns_summarizable(switch: bool) -> impl Summary {
    //     if switch {
    //         NewsArticle {
    //             headline: String::from(
    //                 "Penguins win the Stanley Cup Championship!",
    //             ),
    //             location: String::from("Pittsburgh, PA, USA"),
    //             author: String::from("Iceburgh"),
    //             content: String::from(
    //                 "The Pittsburgh Penguins once again are the best \
    //                  hockey team in the NHL.",
    //             ),
    //         }
    //     } else {
    //         Tweet {
    //             username: String::from("horse_ebooks"),
    //             content: String::from(
    //                 "of course, as you probably already know, people",
    //             ),
    //             reply: false,
    //             retweet: false,
    //         }
    //     }
    // }

    // Using trait bounds to conditionnally implement methods
    use std::fmt::Display;

    struct Pair<T> {
        x: T,
        y: T,
    }

    impl<T> Pair<T> {
        fn new(x: T, y: T) -> Self {
            Self { x, y }
        }
    }

    impl<T: Display + PartialOrd> Pair<T> {
        fn cmp_display(&self) {
            if self.x >= self.y {
                println!("The largest member is x = {}", self.x);
            } else {
                println!("The largest member is y = {}", self.y);
            }
        }
    }

    // Lifetimes validation references

    // It won t compile
    // let r;

    // {
    //     let x = 5;
    //     r = &x;
    // }

    // println!("r: {r}");

    // Borrow Checker in rust compiler
    // fn main() {
    //     let r;                // ---------+-- 'a
    //                           //          |
    //     {                     //          |
    //         let x = 5;        // -+-- 'b  |
    //         r = &x;           //  |       |
    //     }                     // -+       |
    //                           //          |
    //     println!("r: {r}");   //          |
    // }                         // ---------+

    let x = 5;
    let r = &x;
    println!("r: {r}");

    // Generic lifetimes in fn
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {result}");

    // Lifetime annotations
    // &i32        // a reference
    // &'a i32     // a reference with an explicit lifetime
    // &'a mut i32 // a mutable reference with an explicit lifetime

    // Lifetime annotation restrictions
    // It passes
    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {result}");
    }
    // Lifetime of ref must be the smaller lifetime of 2 args
    // let string1 = String::from("long string is long");
    // let result;
    // {
    //     let string2 = String::from("xyz");
    //     result = longest(string1.as_str(), string2.as_str()); // string2 does not live long enough
    // }
    // println!("The longest string is {result}");

    // Workshop experiment
    // @dev move var declarations within scopes
    let string1 = String::from("WAKAWAKAWAKAWAKA");
    let string3;
    let string2;
    let string4;
    let result;
    {
        string2 = String::from("BRRRRRRRRR");
        {
            string3 = String::from("LALALALALALALALA");
            {
                string4 = String::from("45487212145497975654221");
            }
        }
        result = longest(string2.as_str(), string4.as_str());
    }
    println!("The longest string is {result}");

    // Thinking in term of lifetime

    // Ultimately, lifetime syntax is about connecting the lifetimes of various parameters and return values of functions

    // Lifetime annotations in struct
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();
    let i = ImportantExcerpt {
        part: first_sentence,
    };

    // Static lifetime
    // affected reference can live for the entire duration of the program
    let s: &'static str = "I have a static lifetime.";
    // All string literals have the 'static lifetime,
}

// Longest with lifetime annotation
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

// Won't compile because the param lifetime is not explicit
// fn longest(x: &str, y: &str) -> &str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

// returning value not refering to param won't compile due to dangling ref
// fn longest<'a>(x: &str, y: &str) -> &'a str {
//     let result = String::from("really long string");
//     result.as_str()
// }

// Lifetime Elision
// Historical decision of adding pattern to compiler -> no annotations
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
// The patterns programmed into Rust’s analysis of references are called the lifetime elision rules

// Generic type param, Trait bounds and Lifetime
use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {ann}");
    if x.len() > y.len() { x } else { y }
}
