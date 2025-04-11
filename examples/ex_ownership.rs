#![allow(dead_code, unused_variables, clippy::all)]
// TO RUN: cargo test --example ex_ownership

pub fn sum(elements: _) -> i64 {
    todo!()
}

pub fn slice_after_n(elements: _, n: i64) -> _ {
    for (idx, val) in elements.iter().enumerate() {
        if *val == n {
            return Some(&elements[idx..])
        }
    }

    None
}

pub fn merge(mut a: _, mut b: _) -> _ {
    a.append(&mut b);
    a
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn sum_sound() {
        assert_eq!(sum(&vec![]), 0);
        assert_eq!(sum(&vec![1, 2, 3]), 6);
    }

    #[test]
    fn slice_sound() {
        let result = vec![2i64, 3];
        assert_eq!(slice_after_n(&vec![1, 2, 3], 2), Some(result.as_slice()));
    }

    #[test]
    fn merge_sound() {
        assert_eq!(merge(vec![1, 2], vec![3, 4]), vec![1, 2, 3, 4])
    }

    #[test]
    fn all_together() {
        let a = vec![1, 2];
        let b = vec![3, 4];

        assert_eq!(sum(&a), 3);
        assert_eq!(sum(&b), 7);

        let c: Vec<i64> = merge(a, b);
        assert_eq!(sum(&c), 10);
    }
}

fn main() {
    todo!()
}
