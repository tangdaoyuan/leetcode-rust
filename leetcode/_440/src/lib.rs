#![allow(dead_code)]

use std::cmp;

struct Solution;
impl Solution {
    pub fn find_kth_number(n: i64, k: i64) -> i64 {
        let mut cur: i64 = 1;
        let mut _k = k - 1;

        while _k > 0 {
            let steps = Solution::find_steps(cur, n);

            if _k >= steps {
                cur += 1;
                _k -= steps;
            } else {
                cur *= 10;
                _k -= 1;
            }
        }

        return cur;
    }

    pub fn find_steps(cur: i64, n: i64) -> i64 {
        let mut left = cur;
        let mut right = cur;
        let mut steps = 0;

        while left <= n {
            steps += cmp::min(right, n) - left + 1;
            left = left * 10;
            right = right * 10 + 9;
        }

        return steps;
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn find_kth_number_works() {
        let ans = Solution::find_kth_number(681692778, 351251360);
        assert_eq!(ans, 416126219);
    }
}
