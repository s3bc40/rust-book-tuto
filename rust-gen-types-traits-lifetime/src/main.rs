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
}
