#![allow(dead_code)]
struct Solution;
impl Solution {
    pub fn partition_disjoint(nums: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut left_max = nums[0];
        let mut _max = left_max;

        for (ind, val) in nums.iter().enumerate() {
            _max = _max.max(*val);
            if left_max > *val {
                left_max = _max;
                left = ind;
            }
        }

        return left as i32 + 1;
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut nums = vec![5, 0, 3, 8, 6];
        let mut result = Solution::partition_disjoint(nums);
        assert_eq!(result, 3);
        nums = vec![1, 1, 1, 0, 6, 12];
        result = Solution::partition_disjoint(nums);
        assert_eq!(result, 4);
    }
}
