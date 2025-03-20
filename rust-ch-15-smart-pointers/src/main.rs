use crate::List::{Cons, Nil};

// Def of a cons list
enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn main() {
    // Box pointer to the stack, the data 5 to the heap
    let b = Box::new(5);
    println!("b = {b}");

    // Enabling Recursive Types with Boxes
    // cons list -> (1, (2, (3, Nil)))
    // @dev each item in a cons list contains two elements: the value of the current item and the next item.
    // let list = Cons(1, Cons(2, Cons(3, Nil))); // it fails infinite size
    // Rust can’t figure out how much space it needs to store a List value.

    // Breaking the infinite error
    let _list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    // Boxes provide only the indirection and heap allocation
}
