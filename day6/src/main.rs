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

    let x = String::from("diu nei");
    let y = String::from("lo mei");
    println!("longest: {}", longest_with_an_announcement(&x, &y, String::from("diu")));

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

use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmd_display(&self) {
        if self.x >= self.y {
            println!("The largest memeber is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn longest_first<'a>(x: &'a str, y: &str) -> &'a str {
    x
}

// Lifetime Annotations in Struct Definitions
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn important_excerpt() {
    let novel = String::from("Call me Ishmael. Some year ago...");
    let first_sentence = novel.split('.').next().expect("Could no find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
    ) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

