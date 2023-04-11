use std::time::Instant;
use std::env;
use std::fs;

use reqwest::Error;
use reqwest::header;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: {} [filename]", args[0]);
        return;
    }

    let filename = &args[1];

    //let f = fs::read_to_string("/home/alfredtso/Downloads/input.txt");
    match get_input().await {
        Ok(s) => println!("dim: {}", s),
        Err(error) => println!("error: {}", error),
    }

    let f = fs::read_to_string(filename);
    let start_time = Instant::now();
    let fi = f.expect("fds");
    //part_one(&fi);
    let end_time = Instant::now();
    let elapsed_time = end_time - start_time;
    println!("Elapsed time: {:?} millis", elapsed_time.as_millis());

}

pub async fn get_input() -> Result<String, Error> {
    println!("making request");
    let mut headers = header::HeaderMap::new();
    let res = reqwest::get("https://adventofcode.com/2022/day/1/input").await?;
    
    eprintln!("Res: {:?} {}", res.version(), res.status());
    eprintln!("Headers: {:?}\n", res.headers());

    let body = res.text().await?;
    println!("body = {:?}", body);

    return Ok(body)
}

type Inventory = Vec<u32>;

pub fn parse_input(input: &str) -> Vec<Inventory> {
    return input.split("\n\n")
        .map(|x| {x.lines().map(|v| v.parse::<u32>().unwrap())
            .collect()
        })
        .collect();
}

pub fn part_one(input: &str) -> Option<u32> {
    let elfs = parse_input(input);
    let most_calories = elfs.iter().map(|x| x.iter().sum()).max().unwrap();
    println!("highest number: {:?}", Some(most_calories).unwrap());
    return Some(most_calories);
}
