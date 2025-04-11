#![allow(dead_code, unused_variables)]
// TO RUN: cargo test --example ex_factorial

fn factorial_loop(mut input: i64) -> i64 {
    todo!()
}

fn factorial_recursion(input: i64) -> i64 {
    todo!()
}


#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn zero_with_loop() {
        assert_eq!(factorial_loop(0), 1);
    }

    #[test]
    fn one_with_loop() {
        assert_eq!(factorial_loop(1), 1);
    }

    #[test]
    fn two_with_loop() {
        assert_eq!(factorial_loop(2), 2);
    }

    #[test]
    fn five_with_loop() {
        assert_eq!(factorial_loop(5), 120);
    }

    #[test]
    fn zero_with_recursion() {
        assert_eq!(factorial_recursion(0), 1);
    }

    #[test]
    fn one_with_recursion() {
        assert_eq!(factorial_recursion(1), 1);
    }

    #[test]
    fn two_with_recursion() {
        assert_eq!(factorial_recursion(2), 2);
    }

    #[test]
    fn five_with_recursion() {
        assert_eq!(factorial_recursion(5), 120);
    }
}

fn main() {
    println!("Hello world!")
}
