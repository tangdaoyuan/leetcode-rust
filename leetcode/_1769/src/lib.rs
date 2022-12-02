#![allow(dead_code)]
struct Solution;
impl Solution {
    pub fn min_operations(boxes: String) -> Vec<i32> {
        let len = boxes.len();
        let mut dp = vec![0; len];
        let mut ans = vec![0; len];

        let mut prev_count = 0;
        for (ind, ch) in boxes.chars().rev().enumerate() {
            if ind != 0 {
                dp[len - ind - 1] = dp[len - ind] + prev_count;
            }
            if ch == '1' {
                prev_count += 1;
            }
        }

        let mut prev_step = 0;
        prev_count = 0;
        for (ind, ch) in boxes.chars().enumerate() {
            prev_step = prev_step + prev_count;
            ans[ind] = prev_step + dp[ind];
            if ch == '1' {
                prev_count += 1;
            }
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Solution::min_operations(String::from("110")), vec![1, 1, 3]);
        assert_eq!(
            Solution::min_operations(String::from("001011")),
            vec![11, 8, 5, 4, 3, 4]
        );
    }
}
