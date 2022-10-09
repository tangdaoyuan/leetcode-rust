#![allow(dead_code)]

struct Solution;
impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        if nums.len() <= 1 {
            return;
        }
        let mut i: isize = (nums.len() - 2) as isize;
        let mut j = nums.len() - 1;
        let mut k = nums.len() - 1;
        while i.ge(&0) && nums[i as usize] >= nums[j] {
            i -= 1;
            j -= 1;
        }

        if i.ge(&0) {
            while nums[i as usize] >= nums[k] {
                k -= 1;
            }
            nums.swap(i as usize, k);
        }

        i = j as isize;
        j = nums.len() - 1;
        while i < j as isize {
            nums.swap(i as usize, j);
            i += 1;
            j -= 1;
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut nums = vec![1, 2, 3];
        Solution::next_permutation(&mut nums);
        assert_eq!(nums, vec![1, 3, 2]);
        nums = vec![3, 2, 1];
        Solution::next_permutation(&mut nums);
        assert_eq!(nums, vec![1, 2, 3]);
        nums = vec![1, 1, 5];
        Solution::next_permutation(&mut nums);
        assert_eq!(nums, vec![1, 5, 1]);
    }
}
