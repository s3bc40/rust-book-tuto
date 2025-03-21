use crate::List::{Cons, Nil};
use std::ops::Deref;

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

    // Treating Smart Pointers Like Regular References with the Deref Trait
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y); // deref
    // assert_eq!(5, y); // can't compare `{integer}` with `&{integer}`

    // Using Box<T> Like a Reference
    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    // Defining Our Own Smart Pointer
    struct MyBox<T>(T);

    // For new function
    impl<T> MyBox<T> {
        fn new(x: T) -> MyBox<T> {
            MyBox(x)
        }
    }

    // With Deref trait
    impl<T> Deref for MyBox<T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    // assert_eq!(5, *y); // type `MyBox<{integer}>` cannot be dereferenced

    assert_eq!(5, x);
    assert_eq!(5, *y); // it works -> behind the scene *(y.deref())

    // Implicit Deref Coercions with Functions and Methods
    fn hello(name: &str) {
        println!("Hello, {name}!");
    }

    let m = MyBox::new(String::from("Rust"));
    hello(&m);

    // Without rust coercion
    // let m = MyBox::new(String::from("Rust"));
    // hello(&(*m)[..]);

    // How Deref Coercion Interacts with Mutability
    // DerefMut trait to override the * operator on mutable references.

    // Rust does deref coercion when it finds types and trait implementations in three cases:

    // From &T to &U when T: Deref<Target=U>
    // From &mut T to &mut U when T: DerefMut<Target=U>
    // From &mut T to &U when T: Deref<Target=U>
    // The first two cases are the same as each other except that the second implements mutability. The first case states that if you have a &T, and T implements Deref to some type U, you can get a &U transparently. The second case states that the same deref coercion happens for mutable references.

    // The third case is trickier: Rust will also coerce a mutable reference to an immutable one. But the reverse is not possible: immutable references will never coerce to mutable references. Because of the borrowing rules, if you have a mutable reference, that mutable reference must be the only reference to that data (otherwise, the program wouldn’t compile).
}
