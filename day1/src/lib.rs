mod back_of_house;

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() { println!("Compiled!"); }
    }
}

pub fn eat_at_restaurant() {
    // Absolute Path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative Path
    front_of_house::hosting::add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("wheat");
    println!("I'd like {:?} toast please", meal.toast);
}

fn serve_order() {}

mod enum_pub_all {
    #[derive(Debug)]
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_appetizer() {
    let order1 = enum_pub_all::Appetizer::Salad;
    println!("Appetizer: {:?}", order1);
}


// use `use`

mod foo {
    pub mod bar {
        pub fn baz() { println!("called baz"); }
        }

    pub mod bar2 {
        pub fn baz2() { println!("baz2"); }
    }
}

// `use` can be used with absolute and relative path
// use crate::foo::bar;
use self::foo::bar;

pub fn call_baz() {
    bar::baz();
}

// Hashmap
use std::collections::HashMap;

fn test_hash_map() {
    let mut x = HashMap::new();
    x.insert(1,2);
}

// re exporting: external code can use bar2 as if bar2 is in their scope
pub use self::foo::bar2;
