#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn can_partition_k_subsets(nums: Vec<i32>, k: i32) -> bool {
        let mut _nums = nums.clone();
        _nums.sort();
        let sum: i32 = _nums.iter().sum();
        let avg = sum as f32 / k as f32;
        if avg.fract() != 0.0 {
            return false;
        }
        let _avg = avg as i32;
        let mut dp: Vec<bool> = vec![true; 1 << _nums.capacity()];

        fn dfs(s: usize, p: i32, _avg: i32, dp: &mut Vec<bool>, _nums: &Vec<i32>) -> bool {
            if s == 0 {
                return true;
            }
            if !dp[s] {
                return dp[s];
            }
            dp[s] = false;
            for i in 0.._nums.capacity() {
                if _nums[i] + p > _avg {
                    break;
                }
                if ((s >> i) & 1) != 0 {
                    if dfs(s ^ (1 << i), (p + _nums[i]) % _avg, _avg, dp, _nums) {
                        return true;
                    }
                }
            }
            false
        }

        dfs(dp.capacity() - 1, 0, _avg, &mut dp, &_nums)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_partition_k_subsets_works() {
        let nums = vec![4, 3, 2, 3, 5, 2, 1];
        let k = 4;
        let ret = Solution::can_partition_k_subsets(nums, k);
        assert_eq!(ret, true);
    }
}
