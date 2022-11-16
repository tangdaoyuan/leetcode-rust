#![allow(dead_code)]
struct Solution;
impl Solution {
    pub fn is_ideal_permutation(nums: Vec<i32>) -> bool {
        let len = nums.len();
        if len <= 2 {
            return true;
        }
        let mut min_val = nums[len - 1];

        for i in (0..=len - 3).rev() {
            if nums[i] > min_val {
                return false;
            }
            min_val = min_val.min(nums[i + 1]);
        }

        true
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut nums = vec![1, 2, 0];
        let mut result = Solution::is_ideal_permutation(nums);
        assert_eq!(result, false);
        nums = vec![1, 0, 2];
        result = Solution::is_ideal_permutation(nums);
        assert_eq!(result, true);
        nums = vec![0];
        result = Solution::is_ideal_permutation(nums);
        assert_eq!(result, true);
    }
}
