#![allow(dead_code)]

use std::collections::HashSet;
struct Solution;
impl Solution {
    pub fn num_different_integers(word: String) -> i32 {
        let mut set = HashSet::new();

        let mut segments = vec![];
        for (_pos, ch) in word.chars().enumerate() {
            if ch.is_numeric() {
                if segments.len() == 1 {
                    if ch == '0' && segments.first().unwrap() == &'0' {
                        continue;
                    }
                    segments.push(ch);
                } else {
                    segments.push(ch);
                }
            } else {
                if !segments.is_empty() {
                    if segments.len() > 1 && segments.first() == Some(&'0') {
                        segments.remove(0);
                    }
                    set.insert(segments.iter().collect::<String>());
                    segments.clear();
                }
            }
        }

        if !segments.is_empty() {
            if segments.len() > 1 && segments.first() == Some(&'0') {
                segments.remove(0);
            }
            set.insert(segments.iter().collect::<String>());
        }

        set.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut result = Solution::num_different_integers(String::from("a123bc34d8ef34"));
        assert_eq!(result, 3);
        result = Solution::num_different_integers(String::from("leet1234code234"));
        assert_eq!(result, 2);
        assert_eq!(
            Solution::num_different_integers(String::from("a1b01c001")),
            1
        );
        assert_eq!(Solution::num_different_integers(String::from("0a0")), 1);
    }
}
