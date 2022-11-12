#![allow(dead_code)]

static MOD: i32 = (10 as i32).pow(9) + 7;
struct Solution;
impl Solution {
    pub fn num_tilings(n: i32) -> i32 {
        let mut dp = vec![vec![0; 4]; n as usize + 1];

        dp[0][3] = 1;
        for i in 1..=n as usize {
            dp[i][0] = dp[i - 1][3] % MOD;
            dp[i][1] = (dp[i - 1][0] + dp[i - 1][2]) % MOD;
            dp[i][2] = (dp[i - 1][0] + dp[i - 1][1]) % MOD;
            let t1 = (dp[i - 1][0] % MOD + dp[i - 1][1] % MOD) % MOD;
            let t2 = dp[i - 1][2] % MOD;
            let t3 = dp[i - 1][3] % MOD;
            dp[i][3] = (t1 + (t2 + t3) % MOD) % MOD;
        }
        dp[n as usize][3]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = Solution::num_tilings(30);
        assert_eq!(result, 312342182);
    }
}
