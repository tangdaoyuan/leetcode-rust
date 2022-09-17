#![allow(dead_code)]

use std::{cmp::max, collections::HashMap};

struct Solution;

impl Solution {
    pub fn max_length_between_equal_characters(s: String) -> i32 {
        let mut map: HashMap<char, i32> = HashMap::new();
        let mut count: i32 = -1;
        s.chars().into_iter().enumerate().for_each(|(i, f)| {
            if map.get(&f).is_none() {
                map.insert(f, i as i32);
            }
            let ind = map.get(&f).unwrap();
            count = max(i as i32 - *ind - 1, count);
        });

        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let str = String::from("aa");
        let mut ret = Solution::max_length_between_equal_characters(str);
        assert_eq!(ret, 0);
        ret = Solution::max_length_between_equal_characters(String::from("abca"));
        assert_eq!(ret, 2);
        ret = Solution::max_length_between_equal_characters(String::from("cbzxy"));
        assert_eq!(ret, -1);
        ret = Solution::max_length_between_equal_characters(String::from("cabbac"));
        assert_eq!(ret, 4);
    }
}
