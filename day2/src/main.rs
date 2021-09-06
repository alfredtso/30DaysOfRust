fn main() {
    // usually we do it with vec!
    // let v: Vec<i32> = Vec::new();
    
    let v = vec![1, 2, 3];
    println!("Hello, vector {:?}", v);

    // now build a mutable ones
    let mut v1 = Vec::new();
    v1.push(7);
    v1.push(8);
    v1.push(9);
    v1.push(10);
    v1.push(11);
    println!("Hello, v1: {:?}", v1);

    // Reading an element
    // can omit i32
    // indexing like this can cause panic, use it when we want the program to crach if there's an
    // attempt to access an element past the end of the vector
    let third = &v1[2];

    // get method return Option; safe
    match v1.get(2) {
        Some(third) => println!("The third element of v1: {}", third),
        None => println!("There is no third"),
    }

    // iterate over the values in a Vec
    for i in &v1 {
        println!("{}", i);
    }

    println!("Now increment each element by 1 using dereference operator");
    for i in &mut v1 {
        *i += 1;
        println!("{}",i);
    }

    // Enum
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(2.56),
    ];

    // String

}
