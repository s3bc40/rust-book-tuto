use std::thread;
use std::time::Duration;

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    // Capturing the Environment with Closures
    // @dev fn can not capture their environment
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );

    // Closure Type Inference and Annotation
    let expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    // Difference in synxtax
    // fn  add_one_v1   (x: u32) -> u32 { x + 1 }
    // let add_one_v2 = |x: u32| -> u32 { x + 1 };
    // let add_one_v3 = |x|             { x + 1 };
    // let add_one_v4 = |x|               x + 1  ;

    // Compiler infer type from the first usage, hence error at integer
    let example_closure = |x| x;

    let s = example_closure(String::from("hello"));
    // let n = example_closure(5); // this fail

    // Capturing References or Moving Ownership
    // Immutable ref
    println!("Immutable ref:");
    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    let only_borrows = || println!("From closure: {list:?}");

    println!("Before calling closure: {list:?}");
    only_borrows();
    println!("After calling closure: {list:?}");
    println!("");

    // Mutable ref
    println!("Mutable ref:");
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    let mut borrows_mutably = || list.push(7);
    // println!("Before calling closure: {list:?}"); // immutable borrow occurs here
    borrows_mutably();
    println!("After calling closure: {list:?}");
    println!("");

    // We don’t use the closure again after the closure is called, so the mutable borrow ends.
    // Between the closure definition and the closure call, an immutable borrow to print isn’t allowed
    // because no other borrows are allowed when there’s a mutable borrow.

    // Closure takes ownership `move`
    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    thread::spawn(move || println!("From thread: {list:?}"))
        .join()
        .unwrap();
    // println!("{list:?}");
    println!("");

    // Moving Captured Values Out of Closures and the Fn Traits
    // @dev FnOnce, FnMut, Fn
    // Def of `unwrap_or_else`
    // impl<T> Option<T> {
    //     pub fn unwrap_or_else<F>(self, f: F) -> T
    //     where
    //         F: FnOnce() -> T
    //     {
    //         match self {
    //             Some(x) => x,
    //             None => f(),
    //         }
    //     }
    // }

    // `sort_by_key` method with FnMut
    let mut list = [
        Rectangle {
            width: 10,
            height: 1,
        },
        Rectangle {
            width: 3,
            height: 5,
        },
        Rectangle {
            width: 7,
            height: 12,
        },
    ];

    list.sort_by_key(|r| r.width);
    println!("{list:#?}");
    println!("");

    // Compiling error if we use closure that impl FnOnce with sort
    // let mut sort_operations = vec![];
    // let value = String::from("closure called");

    // list.sort_by_key(|r| {
    //     sort_operations.push(value); // takes ownership of value, can't be called again! -> FnOnce then
    //     r.width
    // });
    // println!("{list:#?}");

    // Counting this way works
    let mut num_sort_operations = 0;
    list.sort_by_key(|r| {
        num_sort_operations += 1;
        r.width
    });
    println!("{list:#?}, sorted in {num_sort_operations} operations");
}
