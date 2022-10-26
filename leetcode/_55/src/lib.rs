#![allow(dead_code)]

struct Solution;
impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let len = nums.len();

        if len == 1 {
            return true;
        }

        let mut distance = 0;
        for i in 0..len - 1 {
            if distance < i {
                return false;
            }
            distance = distance.max(i + nums[i] as usize);
            if distance >= len - 1 {
                return true;
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut nums = vec![2, 3, 1, 1, 4];
        let result = Solution::can_jump(nums);
        assert_eq!(result, true);
        nums = vec![3, 2, 1, 0, 4];
        assert_eq!(Solution::can_jump(nums), false);
    }
}
