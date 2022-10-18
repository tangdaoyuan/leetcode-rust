#![allow(dead_code)]

struct Solution;
impl Solution {
    pub fn find_duplicate(nums: Vec<i32>) -> i32 {
        let mut slow = 0;
        let mut fast = 0;
        slow = nums[slow] as usize;
        fast = nums[nums[fast] as usize] as usize;
        while slow != fast {
            slow = nums[slow] as usize;
            fast = nums[nums[fast] as usize] as usize;
        }

        slow = 0;

        while slow != fast {
            slow = nums[slow] as usize;
            fast = nums[fast] as usize;
        }
        return slow as i32;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let nums = vec![1, 3, 4, 2, 2];
        let result = Solution::find_duplicate(nums);
        assert_eq!(result, 2);
    }
}
