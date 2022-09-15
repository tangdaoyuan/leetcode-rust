#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn flip_lights(n: i32, presses: i32) -> i32 {
        if presses > 2 && n > 2 {
            return 8;
        }
        if n < 3 {
            return 1 + ((presses > 0) as i32) * n + (presses > 1 && n == 2) as i32;
        }
        1 + 3 * presses
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn flip_lights_works() {
        let params = vec![(1, 1), (2, 1), (3, 1), (4, 2), (4, 3)];
        let expects = vec![2, 3, 4, 7, 8];

        for (ind, presses) in params.iter().enumerate() {
            let (n, presses) = presses;
            let result = Solution::flip_lights(*n, *presses);
            assert_eq!(result, expects[ind]);
        }
    }
}
