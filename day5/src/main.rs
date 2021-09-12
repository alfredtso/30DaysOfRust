use std::io::Read;
use std::io;
use std::fs::File;

fn main() {
    // manually call panic
    // panic!("crash and burn");

    let v = vec![1,2];
    // v[3];

    // Recoverable errors with Resutl
    use std::fs::File;
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        //Err(error) => panic!("Error opening the file: {:?}", error),
        Err(error) => match error.kind() {
            std::io::ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem create file: {:?}", e),
                },
            other_error => {
                panic!("Problem opening fiel: {:?}", other_error)
            }
        },
    };

    // unwrap
    let f2 = File::open("hello2.txt").unwrap();

    // expect
    let f3 = File::open("hello2.txt").expect("File Not FOUND !");

    // Propagating error

}

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn read_username_from_file_v2() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello3.txt")?.read_to_string(&mut s)?;

    Ok(s)
}
