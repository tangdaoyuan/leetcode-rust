#![allow(dead_code)]

struct Solution;
impl Solution {
    pub fn find_disappeared_numbers(mut nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len() as i32;

        for i in 0..n {
            let ind = (nums[i as usize] - 1) % n;
            nums[ind as usize] = nums[ind as usize] + n;
        }
        
        let mut ret: Vec<i32> = vec![];
        
        for (ind, val) in nums.iter().enumerate() {
            if *val <= n {
                ret.push(ind as i32 + 1);
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
        let mut nums = vec![4,3,2,7,8,2,3,1];
        let mut result = Solution::find_disappeared_numbers(nums);
        assert_eq!(result, vec![5,6]);
        nums = vec![1,1];
        result  = Solution::find_disappeared_numbers(nums);
        assert_eq!(result, vec![2]);
    }
}
