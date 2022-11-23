#![allow(dead_code)]

use std::collections::HashMap;

struct Solution;
impl Solution {
    pub fn count_balls(low_limit: i32, high_limit: i32) -> i32 {
        let mut map = HashMap::new();
        let mut max_val = 0;
        for i in low_limit..=high_limit {
            let mut sum = 0;
            let mut index = i;
            while index != 0 {
                sum += index % 10;
                index = index / 10;
            }
            if !map.contains_key(&sum) {
                map.insert(sum, 0);
            }
            let val = map.get(&sum).unwrap() + 1;
            map.insert(sum, val);
            max_val = max_val.max(val);
        }
        max_val
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = Solution::count_balls(1, 10);
        assert_eq!(result, 2);
    }
}
