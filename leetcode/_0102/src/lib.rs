#![allow(dead_code)]

use std::collections::HashMap;

struct Solution;
impl Solution {
    pub fn check_permutation(s1: String, s2: String) -> bool {
        let mut map: HashMap<char, i32> = HashMap::new();
        s1.chars().for_each(|f| {
            let r = map.get(&f).or(Some(&0)).unwrap();
            map.insert(f, *r + 1);
        });
        s2.chars().all(|f| {
            let r = map.get(&f).or(Some(&0)).unwrap();
            if *r > 0 {
                map.insert(f, *r - 1);
                return true;
            }
            false
        })
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut s1 = String::from("abc");
        let mut s2 = String::from("bca");
        let result = Solution::check_permutation(s1, s2);
        assert_eq!(result, true);
        s1 = String::from("abc");
        s2 = String::from("bad");
        let result = Solution::check_permutation(s1, s2);
        assert_eq!(result, false)
    }
}
