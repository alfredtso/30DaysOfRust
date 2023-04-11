pub fn add(left: usize, right: usize) -> usize {
    left + right
}

mod Solutions {

    mod SearchInsertPosition {
        fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
            return 0;
        }
    }


}

fn main() {
    println!("Diu");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
