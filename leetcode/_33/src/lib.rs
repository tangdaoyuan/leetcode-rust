#![allow(dead_code)]

struct Solution;
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        if nums[0] == target {
            return 0;
        }
        let mut left = 0;
        let mut right = nums.len() - 1;
        while left <= right {
            let mid = (right + left) / 2;
            if target == nums[mid as usize] {
                return mid as i32;
            }
            if nums[left as usize] <= nums[mid as usize] {
                if nums[left as usize] <= target && target < nums[mid as usize] {
                    right = mid - 1;
                } else {
                    left = mid + 1;
                }
            } else {
                if nums[mid as usize] < target && target <= nums[right as usize] {
                    left = mid + 1;
                } else {
                    right = mid - 1;
                }
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut nums = vec![4, 5, 6, 7, 0, 1, 2];
        let mut target = 0;
        let mut result = Solution::search(nums, target);
        assert_eq!(result, 4);
        nums = vec![4, 5, 6, 7, 0, 1, 2];
        target = 3;
        result = Solution::search(nums, target);
        assert_eq!(result, -1);
        nums = vec![1];
        target = 0;
        result = Solution::search(nums, target);
        assert_eq!(result, -1);
    }
}
