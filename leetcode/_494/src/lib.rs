#![allow(dead_code)]

struct Solution;
impl Solution {
    pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
        fn dfs(nums: &Vec<i32>, target: i32, pos: usize) -> i32 {
            if pos == nums.len() {
                return match target {
                    0 => 1,
                    _ => 0,
                };
            }

            let count1 = dfs(nums, target + nums[pos], pos + 1);
            let count2 = dfs(nums, target - nums[pos], pos + 1);

            return count1 + count2;
        }
        return dfs(&nums, target, 0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut nums = vec![1, 1, 1, 1, 1];
        let mut target = 3;
        let mut result = Solution::find_target_sum_ways(nums, target);
        assert_eq!(result, 5);
        nums = vec![1];
        target = 1;
        result = Solution::find_target_sum_ways(nums, target);
        assert_eq!(result, 1);
    }
}
