#![allow(dead_code)]

use std::collections::HashSet;
struct Solution;
impl Solution {
    pub fn split_array_same_average(_nums: Vec<i32>) -> bool {
        if _nums.len() <= 1 {
            return false;
        }
        let mut nums = _nums.clone();
        let n = _nums.len();
        let m = n / 2;
        let mut sum = 0;

        for num in _nums {
            sum += num;
        }

        nums.iter_mut().for_each(|num| {
            *num = *num * n as i32 - sum;
        });

        let mut left = HashSet::new();

        for i in 1..1 << m as i32 {
            let mut tot = 0;
            for j in 0..m {
                if i & (1 << j) != 0 {
                    tot += nums[j];
                }
            }
            if tot == 0 {
                return true;
            }
            left.insert(tot);
        }

        let mut rsum = 0;
        for i in m..n {
            rsum += nums[i];
        }
        for i in 1..(1 << (n - m) as i32) {
            let mut tot = 0;
            for j in m..n {
                if (i & (1 << (j - m))) != 0 {
                    tot += nums[j];
                }
            }
            if tot == 0 || (rsum != tot && left.contains(&(-tot))) {
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
        let mut nums = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let mut result = Solution::split_array_same_average(nums);
        assert_eq!(result, true);
        nums = vec![3, 1];
        result = Solution::split_array_same_average(nums);
        assert_eq!(result, false);
    }
}
