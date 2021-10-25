mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

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

    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }

    fn cook_order() {}
}

use crate::front_of_house::hosting;
use crate::front_of_house::hosting as another_hosting;
pub use crate::front_of_house::hosting as another2_hosting;
use rand::Rng;
use std::collections::HashMap;
use std::{cmp::Ordering, io};
// use std::io::{self, Write};
use std::collections::*;

pub fn serve_order() {}
