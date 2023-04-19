use std::time::Instant;
use std::env;
use std::fs;

use reqwest::Error;
use reqwest::header;
use reqwest::Url;

use http::Method;

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

    //let f = fs::read_to_string(filename);
    let f = get_input().await;
    let start_time = Instant::now();
    let fi = f.expect("fds");
    part_one(&fi);
    let end_time = Instant::now();
    let elapsed_time = end_time - start_time;
    println!("Elapsed time: {:?} millis", elapsed_time.as_millis());

}

pub async fn get_input() -> Result<String, Error> {
    println!("making request");
    let mut headers = header::HeaderMap::new();
    //let res = reqwest::get("https://adventofcode.com/2022/day/1/input").await?;
    let url = Url::parse("https://adventofcode.com/2022/day/1/input").unwrap();
    let res = reqwest::Client::new()
        .request(Method::GET, url)
        .header("Cookie", "session=53616c7465645f5f9bfa85fb001f6ccb7786754993a06f2aac0df50c03b570b45f53111e41e60d2950c3b4a7fc0938545519f14a6d2eab4289d2bf720ad8190b")
        .send()
        .await?;

    println!("Res: {:?} {}", res.version(), res.status());
    println!("Headers: {:?}\n", res.headers());

    let body = res.text().await?;

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
