#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn min_add_to_make_valid(s: String) -> i32 {
        if s == String::from("") {
            return 0;
        }

        let mut count = 0;
        let mut ans = 0;
        for i in s.chars() {
            if i == '(' {
                count += 1;
            } else {
                count -= 1;
            }

            if count < 0 {
                count = 0;
                ans += 1;
            }
        }
        ans + count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut s = String::from("())");
        let mut result = Solution::min_add_to_make_valid(s);
        assert_eq!(result, 1);
        s = String::from("(((");
        result = Solution::min_add_to_make_valid(s);
        assert_eq!(result, 3);
    }
}
