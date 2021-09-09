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

}
