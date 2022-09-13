#![allow(dead_code)]

/// Copy from: https://leetcode.cn/problems/special-array-with-x-elements-greater-than-or-equal-x/

struct Solution;

impl Solution {
    pub fn special_array(nums: Vec<i32>) -> i32 {
        let mut _nums: Vec<i32> = nums[0..].to_vec();
        _nums.sort();
        let len = _nums.len();
        if len == 0 {
            return 0;
        }

        let mut ret: i32 = -1;
        for i in (0..len).rev() {
            let val = i as i32;
            if _nums[len - 1 - i] >= val + 1 {
                if len - 1 - i == 0 {
                    ret = val + 1;
                    break;
                } else if _nums[len - i - 2] < val + 1 {
                    ret = val + 1;
                    break;
                }
            }
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn special_array_works() {
        let vec = vec![3, 5];
        let result = Solution::special_array(vec);
        assert_eq!(result, 2)
    }
}
