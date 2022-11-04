#![allow(dead_code)]
struct Solution;
impl Solution {
    pub fn reach_number(target: i32) -> i32 {
        let _target = target.abs();
        let mut s = 0;
        let mut n = 0;
        while s < _target || (s - target) % 2 == 1 {
            n += 1;
            s += n;
        }
        n
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut target = 2;
        let mut result = Solution::reach_number(target);
        assert_eq!(result, 3);
        target = 3;
        result = Solution::reach_number(target);
        assert_eq!(result, 2);
    }
}
