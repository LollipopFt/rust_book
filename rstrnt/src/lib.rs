#![allow(dead_code, unused_variables)]
/* we can call multiple std modules in 1 line, as such:
use std::cmp::Ordering;  ==>  use std::{cmp::Ordering, io};
use std::io;

use std::io;  ==>  use std::io{self, Write};
use std::io::Write;

bringing all public items defined in a path into scope:
use std::collections::*

using external packages, e.g. use rand::Rng;:
write absolute path, i.e. name of crate                    */

mod housefront { // does not have to be public since it is siblings with eat_at_rstrnt(defined in the same module)
    pub mod hosting {
        pub fn add_waitlist() {}
        fn sit_at_table() {}
    }
    mod serving {
        fn order() {}
        fn serve() {}
        fn pay() {}
    }
}

mod houseback {
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order(); // same as cd ..
    }

    fn cook_order() {}

    pub struct Breakfast { // must pub all values needed explicitly
        pub toast: String, // toast can be chosen by customer: see fn eat_at_rstrnt meal value(available for pub)
        season_fruit: String // customer cannot choose fruit: private
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast { // struct needs public function to create Breakfast instance in
                                                  // eat_at_rstrnt since there is private value (season_fruit) in struct
            Breakfast {
                toast: String::from(toast),
                season_fruit: String::from("peach")
            }
        }
    }

    pub enum Appetizer { // once enum is pub, all values are pub
        Soup,
        Salad,
    }
}

use crate::housefront::hosting;   // using absolute path
// use self::housefront::hosting; => using relative path
// can also do: <use crate::housefront::hosting as chh> -> to call: <chh::add_waitlist()>
// pub use crate::housefront::hosting; => re-exporting: external code can call this, not just limited to eat_at_rstrnt

mod example; // calls example.rs
use crate::example::example_call; // for this instance, example::example_call;(relative path) also works: same src file

fn serve_order() {}
fn eat_at_rstrnt() { // private can call from public but not the other way round
    crate::housefront::hosting::add_waitlist(); // absolute path
    housefront::hosting::add_waitlist(); // relative path
    hosting::add_waitlist(); // using 'use' keyword
    // note: call parent function (hosting) to specify it is from outside module

    let mut meal = houseback::Breakfast::summer("rye"); // breakfast with rye bread
    meal.toast = String::from("wheat"); // change rye to wheat bread
    // meal.season_fruit = String::from("berries"); -> error: private field
    println!("I'd like {} toast please.", meal.toast);

    let appe1 = houseback::Appetizer::Soup; // can be used since pub in enum applies to all values
    let appe2 = houseback::Appetizer::Salad;

    example_call::print();
}