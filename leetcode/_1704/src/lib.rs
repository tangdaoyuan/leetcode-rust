#![allow(dead_code)]

use std::collections::HashSet;
struct Solution;
impl Solution {
    pub fn halves_are_alike(s: String) -> bool {
        let vowels = HashSet::from(['a', 'e', 'i', 'o', 'u']);
        let len = s.len();
        let _s = s.to_ascii_lowercase();
        let a = &_s[0..len / 2];
        let b = &_s[len / 2..];

        let count_a = a.chars().fold(0, |acc, ch| {
            if vowels.contains(&ch) {
                return acc + 1;
            }
            return acc;
        });

        let count_b = b.chars().fold(0, |acc, ch| {
            if vowels.contains(&ch) {
                return acc + 1;
            }
            return acc;
        });

        count_a == count_b
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut s = String::from("book");
        let mut result = Solution::halves_are_alike(s);
        assert_eq!(result, true);
        s = String::from("Uo");
        result = Solution::halves_are_alike(s);
        assert_eq!(result, true);
    }
}
