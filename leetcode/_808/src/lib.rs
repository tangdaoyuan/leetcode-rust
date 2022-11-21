#![allow(dead_code)]
struct Solution;
impl Solution {
    pub fn soup_servings(n: i32) -> f64 {
        let len = (n as f32 / 25.0).ceil() as usize;
        if len >= 179 {
            return 1.0;
        }

        let mut dp = vec![vec![0.0; len + 1]; len + 1];
        dp[0][0] = 0.5;
        for i in 1..=len {
            dp[0][i] = 1.0;
        }
        for i in 1..=len {
            for j in 1..=len {
                dp[i][j] = (dp[0.max(i as isize - 4) as usize][j]
                    + dp[0.max(i as isize - 3) as usize][0.max(j as isize - 1) as usize]
                    + dp[0.max(i as isize - 2) as usize][0.max(j as isize - 2) as usize]
                    + dp[0.max(i as isize - 1) as usize][0.max(j as isize - 3) as usize])
                    / 4.0;
            }
        }
        dp[len][len]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Solution::soup_servings(50), 0.625);
        assert_eq!(Solution::soup_servings(100), 0.71875);
    }
}
