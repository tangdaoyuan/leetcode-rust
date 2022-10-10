#![allow(dead_code)]

struct Solution;
impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut ans: Vec<i32> = vec![-1, -1];

        if nums.is_empty() {
            return ans;
        }

        fn search(nums: &Vec<i32>, ans: &mut Vec<i32>, target: i32, flag: bool) {
            let mut left: usize = 0;
            let mut right: usize = nums.len() - 1;
            while left <= right {
                let mut mid = (left + right) / 2;
                if nums[mid] == target {
                    if flag {
                        while mid > 0 && nums[mid - 1] == target {
                            mid -= 1;
                        }
                        ans[0] = mid as i32;
                    } else {
                        while mid < nums.len() - 1 && nums[mid + 1] == target {
                            mid += 1;
                        }
                        ans[1] = mid as i32;
                    }
                    break;
                } else if nums[mid] < target {
                    left = mid + 1;
                } else {
                    if mid == 0 {
                        break;
                    }
                    right = mid - 1;
                }
            }
        }
        search(&nums, &mut ans, target, true);
        search(&nums, &mut ans, target, false);
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut nums = vec![5, 7, 7, 8, 8, 10];
        let mut target = 8;
        let mut result = Solution::search_range(nums, target);
        assert_eq!(result, vec![3, 4]);
        nums = vec![5, 7, 7, 8, 8, 10];
        target = 6;
        result = Solution::search_range(nums, target);
        assert_eq!(result, vec![-1, -1]);
        nums = vec![1];
        target = 0;
        result = Solution::search_range(nums, target);
        assert_eq!(result, vec![-1, -1])
    }
}
