#![allow(dead_code)]
struct Solution;
impl Solution {
    pub fn num_subarray_bounded_max(nums: Vec<i32>, left: i32, right: i32) -> i32 {
        let len = nums.len();

        if len <= 0 {
            return 0;
        }

        let mut l_pos = 0;
        let mut r_pos = 0;
        let mut prev_valid: i32 = -1;
        let mut ans = 0;

        while r_pos < len && l_pos < len {
            if nums[r_pos] > right {
                l_pos = r_pos + 1;
                prev_valid = -1;
            } else if nums[r_pos] <= right && nums[r_pos] >= left {
                prev_valid = r_pos as i32;
                ans += (prev_valid - l_pos as i32) + 1;
            } else {
                if prev_valid != -1 {
                    ans += (prev_valid - l_pos as i32) + 1;
                }
            }
            r_pos += 1;
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut result = Solution::num_subarray_bounded_max(vec![2, 1, 4, 3], 2, 3);
        assert_eq!(result, 3);
        result = Solution::num_subarray_bounded_max(vec![2, 9, 2, 5, 6], 2, 8);
        assert_eq!(result, 7);
        result =
            Solution::num_subarray_bounded_max(vec![73, 55, 36, 5, 55, 14, 9, 7, 72, 52], 32, 69);
        assert_eq!(result, 22);
        result = Solution::num_subarray_bounded_max(
            vec![16, 69, 88, 85, 79, 87, 37, 33, 39, 34],
            55,
            57,
        );
        assert_eq!(result, 0);
    }
}
