#![allow(dead_code)]
struct Solution;
impl Solution {
    pub fn sum_subseq_widths(_nums: Vec<i32>) -> i32 {
        let len = _nums.len();
        let MOD: i64 = (10 as i64).pow(9) + 7;
        let mut nums = _nums.clone();
        nums.sort();
        let mut dp = vec![0; _nums.len()];
        dp[0] = 1;
        for ind in 1..len {
            dp[ind] = dp[ind - 1] * 2 % MOD;
        }

        let mut ans: i64 = 0;
        for i in 0..len {
            ans = ans + (dp[i] - dp[len - i - 1]) as i64 * nums[i] as i64;
        }

        ((ans % MOD + MOD as i64) % MOD) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut nums = vec![2, 1, 3];
        let mut result = Solution::sum_subseq_widths(nums);
        assert_eq!(result, 6);
        nums = vec![2];
        result = Solution::sum_subseq_widths(nums);
        assert_eq!(result, 0);
        nums = vec![
            5, 69, 89, 92, 31, 16, 25, 45, 63, 40, 16, 56, 24, 40, 75, 82, 40, 12, 50, 62, 92, 44,
            67, 38, 92, 22, 91, 24, 26, 21, 100, 42, 23, 56, 64, 43, 95, 76, 84, 79, 89, 4, 16, 94,
            16, 77, 92, 9, 30, 13,
        ];
        result = Solution::sum_subseq_widths(nums);
        assert_eq!(result, 857876214);
    }
}
