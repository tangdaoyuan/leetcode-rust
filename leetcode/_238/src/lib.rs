#![allow(dead_code)]

struct Solution;
impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        if nums.len() < 2 {
            return nums.clone();
        }

        let mut ans = vec![0; nums.len()];
        let mut dp = vec![1; nums.len()];

        dp.push(1);

        for i in 1..nums.len() {
            dp[i] = dp[i - 1] * nums[i - 1]
        }

        let mut right = 1;
        for i in (0..nums.len()).rev() {
            ans[i] = dp[i] * right;
            right = right * nums[i];
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut nums = vec![1, 2, 3, 4];
        let mut result = Solution::product_except_self(nums);
        assert_eq!(result, vec![24, 12, 8, 6]);
        nums = vec![-1, 1, 0, -3, 3];
        result = Solution::product_except_self(nums);
        assert_eq!(result, vec![0, 0, 9, 0, 0])
    }
}
