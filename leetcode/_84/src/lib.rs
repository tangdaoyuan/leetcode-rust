#![allow(dead_code)]
struct Solution;
impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let len = heights.len();
        let mut stack: Vec<i32> = vec![];
        let mut left: Vec<i32> = vec![-1; len];
        let mut right: Vec<i32> = vec![-1; len];

        for ind in 0..len {
            while !stack.is_empty() && heights[*stack.last().unwrap() as usize] >= heights[ind] {
                stack.pop();
            }
            if stack.is_empty() {
                left[ind] = -1
            } else {
                left[ind] = *stack.last().unwrap();
            }
            stack.push(ind as i32);
        }

        stack = vec![];

        for ind in (0..len).rev() {
            while !stack.is_empty() && heights[*stack.last().unwrap() as usize] >= heights[ind] {
                stack.pop();
            }
            if stack.is_empty() {
                right[ind] = len as i32;
            } else {
                right[ind] = *stack.last().unwrap();
            }
            stack.push(ind as i32);
        }

        let mut ans = 0;
        for ind in 0..len {
            ans = ans.max((right[ind] - left[ind] - 1) * heights[ind]);
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let heights = vec![2, 1, 5, 6, 2, 3];
        let result = Solution::largest_rectangle_area(heights);
        assert_eq!(result, 10);
    }
}
