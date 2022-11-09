#![allow(dead_code)]

use std::collections::HashSet;
struct Solution;
impl Solution {
    pub fn order_of_largest_plus_sign(n: i32, mines: Vec<Vec<i32>>) -> i32 {
        let mut dp = vec![vec![n; n as usize]; n as usize];
        let mut set = HashSet::new();
        for mine in mines {
            set.insert(mine[0] * n + mine[1]);
        }
        let mut ans = 0;
        for i in 0..n as usize {
            let mut count = 0;

            // left
            for j in 0..n as usize {
                if set.contains(&((i * n as usize + j) as i32)) {
                    count = 0;
                } else {
                    count += 1;
                }
                dp[i][j] = dp[i][j].min(count);
            }

            count = 0;

            // right
            for j in (0..=(n - 1) as usize).rev() {
                if set.contains(&((i * n as usize + j) as i32)) {
                    count = 0;
                } else {
                    count += 1;
                }
                dp[i][j] = dp[i][j].min(count);
            }
        }

        for i in 0..n as usize {
            let mut count = 0;

            // up
            for j in 0..n as usize {
                if set.contains(&((j * n as usize + i) as i32)) {
                    count = 0;
                } else {
                    count += 1;
                }
                dp[j][i] = dp[j][i].min(count);
            }

            count = 0;

            // down
            for j in (0..=(n - 1) as usize).rev() {
                if set.contains(&((j * n as usize + i) as i32)) {
                    count = 0;
                } else {
                    count += 1;
                }
                dp[j][i] = dp[j][i].min(count);
                ans = ans.max(dp[j][i]);
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
        let mut n = 5;
        let mut mines = vec![vec![4, 2]];
        let result = Solution::order_of_largest_plus_sign(n, mines);
        assert_eq!(result, 2);
    }
}
