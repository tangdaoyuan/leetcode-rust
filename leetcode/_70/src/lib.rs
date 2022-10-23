#![allow(dead_code)]
struct Solution;
impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let mut dp: Vec<i32> = vec![0; n as usize + 1];
        dp[0] = 1;
        dp[1] = 1;
        if n <= 1 {
            return dp[n as usize];
        }

        for i in 2..=(n as usize) {
            dp[i] = dp[i - 1] + dp[i - 2]
        }
        dp[n as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Solution::climb_stairs(2), 2);
        assert_eq!(Solution::climb_stairs(3), 3);
    }
}
