#![allow(dead_code, unused_variables, clippy::all)]
// TO RUN: cargo test --example ex_iterators

use std::collections::HashMap;

fn sum<I: Iterator<Item = i64>>(elements: I) -> i64 {
    todo!()
}

fn get_elements_with_even_keys(elements: &HashMap<i64, String>) -> Vec<String> {
    todo!()
}

fn first_word<I: Iterator<Item = char>>(chars: I) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn sum_sound() {
        assert_eq!(sum(vec![1i64, 2, 3].iter().copied()), 6);
        assert_eq!(sum(vec![3i64, 2, 4].iter().copied()), 9);
    }

    #[test]
    fn get_elements_with_even_keys_sound() {
        let mut map = HashMap::new();
        map.insert(2, "hi".to_owned());
        map.insert(-3, "hello".to_owned());
        map.insert(8, "hey".to_owned());
        map.insert(123, "bye".to_owned());

        let vec = get_elements_with_even_keys(&map);
        assert_eq!(vec.len(), 2);
        assert!(vec.contains(&"hi".to_owned()));
        assert!(vec.contains(&"hey".to_owned()));
    }

    #[test]
    fn first_word_sound() {
        assert_eq!(first_word("Hello world!".chars()), "Hello");
        assert_eq!(first_word("word".chars()), "word");
    }
}

fn main() {
    todo!()
}
