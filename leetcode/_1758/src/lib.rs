#![allow(dead_code)]
struct Solution;
impl Solution {
    pub fn min_operations(s: String) -> i32 {
        let mut prev = '1';
        let mut ans = 0;
        for ch in s.chars() {
            if ch == prev {
                ans += 1;
                prev = if prev == '0' { '1' } else { '0' };
            } else {
                prev = ch;
            }
        }
        ans.min(s.len() as i32 - ans)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Solution::min_operations(String::from("0100")), 1);
        assert_eq!(Solution::min_operations(String::from("10")), 0);
        assert_eq!(Solution::min_operations(String::from("1111")), 2);
    }
}
