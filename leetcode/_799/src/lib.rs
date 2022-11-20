#![allow(dead_code)]
struct Solution;
impl Solution {
    pub fn champagne_tower(poured: i32, query_row: i32, query_glass: i32) -> f64 {
        let len = query_row as usize;
        let mut dp = vec![vec![]; len + 1];
        for i in 0..=len {
            dp[i] = vec![0.0; len + 1];
        }

        dp[0][0] = poured as f64;

        for i in 0..query_row as usize {
            for j in 0..=i {
                if dp[i][j] <= 1.0 {
                    continue;
                }
                dp[i + 1][j] += (dp[i][j] - 1.0) / 2.0;
                dp[i + 1][j + 1] += (dp[i][j] - 1.0) / 2.0;
            }
        }

        dp[query_row as usize][query_glass as usize].min(1.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Solution::champagne_tower(2, 1, 1), 0.5);
        assert_eq!(Solution::champagne_tower(100000009, 33, 17), 1.0);
    }
}
