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
