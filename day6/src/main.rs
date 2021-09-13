fn main() {
    println!("Hello, Day6");

    let number_list = vec![34,50,25,100,150];
    let result1 = largest(&number_list);
    println!("The largest in {:?} is {}", number_list, result1);

    let char_list = vec!['y', 'a', 'c'];
    let result2 = largest(&char_list);
    println!("The largest in {:?} is {}", char_list, result2);


    // test_point_2
    test_point_2();

    // mixup
    let p1 = Point {x: 5, y: 10.4 };
    let p2 = Point {x: "Hello", y: 'c'};
    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}

fn largest<T: PartialOrd + Copy>(list: &[T]) -> &T {

    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = &item;
        }
    }

    largest
}

// Generic struct
struct Point<T, U> {
    x: T,
    y: U,
}

fn test_point() {
    let wont_work = Point {x: 5, y: 4.0};
    
}

// Enum
enum Result<T, E> {
    Ok(T),
    Err(E),
}

struct Point2<T> {
    x: T,
    y: T,
}

impl<T> Point2<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

fn test_point_2() {
    let p = Point2{x:5, y: 10};
    println!("Point2 {}", p.x());
}

// mixup
impl<T,U> Point<T,U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, {}", self.headline, self.location)
    }
}

pub struct Tweet {
    pub usename: String,
    pub content: String,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{} tweeted {}", self.usename, self.content)
    }
}

pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
