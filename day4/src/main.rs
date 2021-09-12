use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 20);
    
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let mut scores: HashMap<_, _> =
        teams.into_iter().zip(initial_scores.into_iter()).collect();

    // println!("score: {:?}", scores);

    let field_name = String::from("Favourite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // try to see if they are valid
    //println!("field name : {}, field value : {}", field_name, field_value); // Won't compile

    // get method
    // let score = map.get(&field_name);
    // println!("score: {}", score); // Option<&String> doesn't has Display trait
    
    // This will print blue and yello in random order
    for (k, v) in &scores {
        println!("{}: {}", k, v);
    }

    // Only insert if key has no value
    let mut only = HashMap::new();
    only.insert(String::from("Blue"), 10);

    only.entry(String::from("Yellow")).or_insert(50);
    only.entry(String::from("Blue")).or_insert(50);
    println!("{:?}", only);

    // update
    let text = "hello world wonderful world";

    let mut up = HashMap::new();

    for word in text.split_whitespace() {
        let count = up.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", up);

}
