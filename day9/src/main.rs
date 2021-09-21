use std::thread;
use std::time::Duration;
use std::collections::HashMap;

/*
fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly ...");
    thread::sleep(Duration::from_secs(2));
    intensity
}
*/

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);
    // testing_closure();
    println!("using_closure");
    day9::using_closure();
    // day9::iter_ex();
}

fn generate_workout(intensity: u32, random_number: u32) {
    // let expensive_call = simulated_expensive_calculation(intensity);
    let mut expensive_call = Cacher::new(|num: u32| -> u32 {
        println!("calculating slowly ...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!(
            "Today, do {} pushups!", expensive_call.value(intensity)
        );
        println!(
            "Next, do {} situps!", expensive_call.value(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!", expensive_call.value(intensity)
            );
        }
    }
}



struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: HashMap<u32, u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: HashMap::new(),
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        let value = self.value.get(&arg);
        match value {
            Some(val) => val.clone(),
            None => {
                let v = (self.calculation)(arg);
                self.value.insert(arg, v);
                v
            },
        }
    }
}

#[cfg(test)]
#[test]
fn call_with_different_values() {
    let mut c = Cacher::new(|a| a);

    let _v1 = c.value(1);
    let v2 = c.value(2);

    assert_eq!(v2, 2);
}


