use std::thread;
use std::time::Duration;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    handle.join().unwrap();
    for i in 1..5 {
        println!("hi number {} from the main thread", i);
        thread::sleep(Duration::from_millis(1));
    }

    // move
    let v = vec![1, 2, 3];
    let move_handle = thread::spawn(move || {
        println!("Here's vector v: {:?}", v);
    });

    // added because sometime the vector would not be printed completely
    move_handle.join().unwrap();
    
    // channel
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
        // try to access the val after sending it down the chanel
        // println!("val is {}", val);
    });

    let msg = rx.recv().unwrap();
    println!("Getting from tx ... {}", msg);

    // Mutex<T>
    let m = Mutex::new(5);

    // acquire the lock
    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m);
    
    // sharing between multiple threads
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
