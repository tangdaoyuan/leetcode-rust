#![allow(dead_code)]
struct Solution;
impl Solution {
    pub fn check(nums: Vec<i32>) -> bool {
        let mut prev = 0;
        let mut count = 0;

        for i in 1..nums.len() {
            if nums[prev] <= nums[i] {
                prev = i;
            } else {
                if count >= 1 {
                    return false;
                }
                count += 1;
                prev = i;
            }
        }
        if count == 1 {
            return nums[nums.len() - 1] <= nums[0];
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut result = Solution::check(vec![3, 4, 5, 1, 2]);
        assert_eq!(result, true);
        result = Solution::check(vec![2, 1, 3, 4]);
        assert_eq!(result, false);
        result = Solution::check(vec![1, 2, 3]);
        assert_eq!(result, true);
    }
}
