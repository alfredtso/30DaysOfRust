use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::io::prelude::*;

fn main() {
    let f = File::open("/tmp/day1.data").unwrap();
    let f = BufReader::new(f);

    for line in f.lines() {
        println!("{}", line.unwrap());
    }
}
