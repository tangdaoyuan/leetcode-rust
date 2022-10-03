#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn check_ones_segment(s: String) -> bool {
        if s.as_str() == "1" {
            return true;
        }
        let mut prev: usize = 0;

        let mut i = 0;
        while i < s.len() {
            if s.chars().nth(i).unwrap() == '1' {
                if prev != i {
                    return false;
                }
                while i + 1 < s.len() && s.chars().nth(i + 1).or(Some('0')) == Some('1') {
                    i += 1;
                }

                prev = i;
            }
            i+=1;
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut s = String::from("1001");
        let result = Solution::check_ones_segment(s);
        assert_eq!(result, false);
        s = String::from("110");
        assert_eq!(Solution::check_ones_segment(s), true);
        s = String::from("10");
        assert_eq!(Solution::check_ones_segment(s), true);
    }
}
