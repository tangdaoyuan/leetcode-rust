#![allow(dead_code)]
#![allow(non_snake_case)]

struct Solution;
impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        if nums.len() <= 0 {
            return vec![];
        }
        let mut _nums = nums.clone();
        _nums.sort();

        let mut ret: Vec<Vec<i32>> = vec![];
        let n = _nums.len();
        for i in 0..n {
            if _nums[0] > 0 {
                return ret;
            }
            if i > 0 && _nums[i] == _nums[i - 1] {
                continue;
            }
            let mut L = i + 1;
            let mut R = n - 1;

            while L < R {
                if _nums[i] + _nums[L] + _nums[R] == 0 {
                    ret.push(vec![_nums[i], _nums[L], _nums[R]]);

                    while L < R && _nums[L] == _nums[L + 1] {
                        L = L + 1;
                    }
                    while L < R && _nums[R] == _nums[R - 1] {
                        R = R - 1;
                    }
                    L = L + 1;
                    R = R - 1;
                } else if _nums[i] + _nums[L] + _nums[R] > 0 {
                    R = R - 1;
                } else {
                    L = L + 1;
                }
            }
        }

        ret
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut nums = vec![-1, 0, 1, 2, -1, -4];
        let mut result = Solution::three_sum(nums);
        assert_eq!(result, vec![vec![-1, -1, 2], vec![-1, 0, 1],]);
        nums = vec![0, 1, 1];
        result = Solution::three_sum(nums);
        assert_eq!(result, vec![] as Vec<Vec<i32>>);
        nums = vec![0, 0, 0];
        result = Solution::three_sum(nums);
        assert_eq!(result, vec![vec![0, 0, 0]]);
    }
}
