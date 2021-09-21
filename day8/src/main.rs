use std::process;
use std::env;

use day8::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    /*
    let query = &args[1];
    let filename = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("Something is wrong with reading the file");

    println!("With text:\n{}", contents);
    */

    // use arg parser
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Error : {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    if let Err(e) = day8::run(config) {
        print!("Application error: {}", e);
        process::exit(2);
    }
}


