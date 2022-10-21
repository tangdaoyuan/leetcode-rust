#![allow(dead_code)]
struct Solution;
impl Solution {
    pub fn max_coins(nums: Vec<i32>) -> i32 {
        let len = nums.len();
        let mut vals = vec![1; len + 2];
        for ind in 1..=len {
            vals[ind] = nums[ind - 1];
        }
        let mut rec = vec![vec![-1; len + 2]; len + 2];

        fn solve(left: usize, right: usize, vals: &Vec<i32>, rec: &mut Vec<Vec<i32>>) -> i32 {
            if left >= right - 1 {
                return 0;
            }
            if rec[left][right] != -1 {
                return rec[left][right];
            }
            for i in left + 1..right {
                let mut sum = vals[left] * vals[i] * vals[right];
                sum += solve(left, i, vals, rec) + solve(i, right, vals, rec);
                rec[left][right] = rec[left][right].max(sum);
            }
            rec[left][right]
        }
        solve(0, len + 1, &vals, &mut rec)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut nums = vec![3, 1, 5, 8];
        let mut result = Solution::max_coins(nums);
        assert_eq!(result, 167);
        nums = vec![1, 5];
        result = Solution::max_coins(nums);
        assert_eq!(result, 10);
    }
}
