// use std::fmt::Result;
// use std::io::Result as IoResult;
use std::{fmt::Result, io::Result as IoResult};
// use std::io;
// use std::io::Write;
use std::io::{self, Write};
// Global import
use std::collections::*;

mod front_of_house;

fn deliver_order() {}

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }

    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}
}

mod customer {
    use crate::front_of_house::hosting;

    pub fn eat_at_restaurant() {
        // Absolute path
        hosting::add_to_waitlist();

        // Relative path
        super::front_of_house::hosting::add_to_waitlist();

        // Order breakfast in the summer with Rye toast
        let mut meal = super::back_of_house::Breakfast::summer("Rye");
        // Change our mind about what bread we'd like
        meal.toast = String::from("Wheat");
        println!("I'd like {} toast please", meal.toast);

        let order1 = super::back_of_house::Appetizer::Soup;
        let order2 = super::back_of_house::Appetizer::Salad;
    }
}

// fn function1() -> Result {
//     // --snip--
// }

// fn function2() -> IoResult<()> {
//     // --snip--
// }
