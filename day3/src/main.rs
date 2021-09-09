fn main() {
    println!("Hello, world!");

    // Create a New String
    let mut s = String::new();

    // Create a String with some initial data
    let data = "some contents";

    let content  = data.to_string();
    let content1 = "some contents 2".to_string();
    let content2 = String::from("some content 3");
    
    // UTF-8 coded so we can include chinese too 
    let hello = String::from("你好");

    // Updating a String with push_str and push
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    // s2 will br printed, meaning that the ownership has not been taken, push_str takes a string
    // slice
    println!("s2 is {}", s2);

    let mut s3 = String::from("lo");
    s3.push('l');
    println!("s3 is {}", s3);

    // Concat with + or format! 
    let s4 = String::from("Hello, ");
    let s5 = String::from("world!");
    let s6 = s4 + &s5;
    println!("s6 is {} and add operator second arg is slice str", s6);

    // for more string we need & to all except the first
    let s7 = String::from("tic");
    let s8 = String::from("tac");
    let s9 = String::from("toe");

    let s10 = s7 + "-" + &s8 + "-" + &s9;
    println!("s10 is {}", s10);
    // or use
    // let s10 = format!("{}-{}-{}", s1, s2, s3);

    // the strings should contain 4 "characters"
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }
}
