#![allow(dead_code)]
struct Solution;
impl Solution {
    pub fn second_highest(s: String) -> i32 {
        let mut first = -1;
        let mut second = -1;
        for ch in s.chars() {
            if ch.is_numeric() {
                let cur = ch.to_digit(10).unwrap() as i32;
                if cur > first {
                    second = first;
                    first = cur;
                } else if cur < first && cur > second {
                    second = cur;
                }
            }
        }

        second
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Solution::second_highest(String::from("dfa12321afd")), 2);
        assert_eq!(Solution::second_highest(String::from("abc1111")), -1);
    }
}
