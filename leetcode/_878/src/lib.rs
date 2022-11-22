#![allow(dead_code)]
struct Solution;
impl Solution {
    pub fn nth_magical_number(_n: i32, _a: i32, _b: i32) -> i32 {
        let n = _n as i64;
        let a = _a as i64;
        let b = _b as i64;
        let mod_val = (10 as i64).pow(9) + 7;
        let lcm = a / Self::gcd(a, b) * b;
        let mut left = 0_i64;
        let mut right = (a as i64).max(b) * n;
        while left + 1 < right {
            let mid = left + (right - left) / 2;
            if mid / a + mid / b - mid / lcm >= n {
                right = mid;
            } else {
                left = mid;
            }
        }
        (right % mod_val) as i32
    }

    pub fn gcd(a: i64, b: i64) -> i64 {
        if b == 0 {
            a
        } else {
            Self::gcd(b, a % b)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = Solution::nth_magical_number(1000000000, 40000, 40000);
        assert_eq!(result, 999720007);
    }
}
