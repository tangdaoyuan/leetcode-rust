#![allow(dead_code)]

use std::{collections::HashMap, iter};
struct Solution;

impl Solution {
    pub fn frequency_sort(nums: Vec<i32>) -> Vec<i32> {
        let mut map: HashMap<i32, i32> = HashMap::new();
        nums.iter().for_each(|f| {
            if map.get(f).is_none() {
                map.insert(*f, 1);
            }
            map.insert(*f, *map.get(f).unwrap() + 1);
        });
        let mut v: Vec<_> = map.into_iter().collect();

        v.sort_by(|x, y| {
            if x.1 == y.1 {
                return y.0.cmp(&x.0);
            }
            x.1.cmp(&y.1)
        });
        let mut ret: Vec<i32> = vec![];
        v.iter().for_each(|f| {
            let mut tmp: Vec<_> = iter::repeat(f.0).take((f.1 - 1) as usize).collect();
            ret.append(&mut tmp);
        });
        ret
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn frequency_sort_works() {
        let mut data = vec![1, 1, 2, 2, 2, 3];
        let mut ret = Solution::frequency_sort(data);
        assert_eq!(ret, vec![3, 1, 1, 2, 2, 2]);
        data = vec![2, 3, 1, 3, 2];
        ret = Solution::frequency_sort(data);
        assert_eq!(ret, vec![1, 3, 3, 2, 2]);
        data = vec![-1, 1, -6, 4, 5, -6, 1, 4, 1];
        ret = Solution::frequency_sort(data);
        assert_eq!(ret, vec![5, -1, 4, 4, -6, -6, 1, 1, 1]);
    }
}
