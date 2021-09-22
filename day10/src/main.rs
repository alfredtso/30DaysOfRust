use std::ops::Deref;
use std::rc::Rc;

use crate::List::{Cons, Nil};
use crate::MyList::{MyCons, MyNil};

enum List {
    Cons(i32, Box<List>),
    Nil,
}


enum MyList {
    MyCons(i32, Rc<MyList>),
    MyNil,
}

struct MyBox<T>(T); 

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CSP with data {}", self.data);
    }
}


fn main() {
    //let list = Cons(1, Box::new(Cons(2, Box::new(Nil))));

    // Dereference Operator
    let x = 5;
    // let y = &x;
    let y = MyBox::new(x);

    assert_eq!(5, *y);

    // Deref coercion
    let m = MyBox::new(String::from("Rust"));
    hello(&m);

    // Drop
    let cdrop = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");

    // early drop
    let e = CustomSmartPointer {
        data: String::from("early"),
    };
    println!("CSP in early drop example created");
    drop(e);
    println!("early drop dropped before end of main()");

    // Reference counting

    let a = Rc::new(MyCons(5, Rc::new(MyCons(10, Rc::new(MyNil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = MyCons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    let c = MyCons(4, Rc::clone(&a));
    println!("count after creating c = {}", Rc::strong_count(&a));
}

fn hello(name: &str) {
    println!("Hello, {}", name);
}
