#![allow(dead_code)]

struct Solution;
impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let len = height.len();

        if len < 2 {
            return 0;
        }

        let mut dp = vec![0; len];
        dp[len - 1] = len - 1;

        for i in (0..len - 1).rev() {
            dp[i] = if height[dp[i + 1]] > height[i] {
                dp[i + 1]
            } else {
                i
            }
        }

        let mut left = 0;
        let mut ans = 0;

        for i in 0..len {
            left = if height[left] > height[i] { left } else { i };
            ans += height[left].min(height[dp[i]]) - height[i];
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut height = vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1];
        let result = Solution::trap(height);
        assert_eq!(result, 6);
        height = vec![4, 2, 0, 3, 2, 5];
        assert_eq!(Solution::trap(height), 9);
    }
}
