/*
fn testing_closure() {
    
    let x = 4;

    fn equal_to_x(z: i32) -> bool {
        z == x
    }

    let y = 4;

    assert!(equal_to_x(y));
}
*/

pub fn using_closure() {
    let x = 4;
    let equal_to_x = |z| z == x;
    let y = 4;

    assert!(equal_to_x(y));
}

#[cfg(test)]
/*
#[test]
#[should_panic]
fn use_var_after_move() {
    let x = vec![1,2,3];

    let equal_to_x = move |z: Vec<i32>| z == x;

    //println!("using x after moving it : {:?}", x);
}


pub fn iter_ex() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("v1: {}", val);
    }
}
*/

#[test]
pub fn iter_ex_next() {
    let v1 = vec![1, 2, 3];
    // notice the mut
    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
}

#[test]
fn iter_sum() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    let total: i32 = v1_iter.sum();

    assert_eq!(total, 6);
}

#[test]
fn iter_adaptor_must_use_with_consum() {
    let v1 = vec![1, 2, 3];
    
    // after map, the iterator will turn into Map which is also iter, we use collect to consume
    let v2: Vec<_> = v1.iter().map(|x| x + 2).collect();
    
    assert_ne!(v1, v2);
}

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

#[test]
fn calling_next_directly() {
    let mut counter = Counter::new();

    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);
}

#[test]
fn using_other_iterator_trait_methods() {
    let sum: u32 = Counter::new()
        .zip(Counter::new().skip(1))
        .map(|(a, b)| a*b)
        .filter(|x| x % 3 == 0)
        .sum();
    assert_eq!(sum, 18);
}




