use std::collections::HashMap;

fn main() {
    day1::eat_appetizer();
    day1::eat_at_restaurant();
    day1::call_baz();
    let mut x: HashMap<u8, u8> = HashMap::new();
    x.insert(1,2);
    day1::bar2::baz2();
}
