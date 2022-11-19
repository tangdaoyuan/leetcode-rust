#![allow(dead_code)]
struct Solution;
impl Solution {
    pub fn largest_altitude(gain: Vec<i32>) -> i32 {
        let mut max_val = 0;
        let mut sum = 0;
        gain.iter().for_each(|h| {
            sum += *h;
            max_val = max_val.max(sum);
        });
        max_val
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut result = Solution::largest_altitude(vec![-5, 1, 5, 0, -7]);
        assert_eq!(result, 1);
        result = Solution::largest_altitude(vec![-4, -3, -2, -1, 4, 3, 2]);
        assert_eq!(result, 0);
    }
}
