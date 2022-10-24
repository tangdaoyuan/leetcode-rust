#![allow(dead_code)]

struct Solution;
impl Solution {
    pub fn num_trees(n: i32) -> i32 {
        let mut dp = vec![0; (n + 1) as usize];
        dp[0] = 1;
        dp[1] = 1;
        for i in 2..=(n as usize) {
            for j in 1..=i {
                dp[i] = dp[i] + dp[j - 1] * dp[i - j];
            }
        }
        dp[n as usize]
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = Solution::num_trees(3);
        assert_eq!(result, 5);
    }
}
