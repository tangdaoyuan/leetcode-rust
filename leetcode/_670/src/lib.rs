#![allow(dead_code)]

use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn maximum_swap(num: i32) -> i32 {
        let mut map: HashMap<i32, usize> = HashMap::new();
        let mut num_vec: Vec<i32> = num
            .to_string()
            .chars()
            .map(|x| -> i32 { x.to_digit(10).unwrap() as i32 })
            .collect();

        for (ind, val) in num_vec.iter().enumerate() {
            let temp = map.get(val);
            match temp {
                Some(v) => {
                    if *v > ind {
                        map.insert(*val, ind);
                    }
                }
                None => {
                    map.insert(*val, ind);
                }
            }
        }

        let mut keys = num_vec.clone();
        keys.sort();
        keys.reverse();

        for (pos1, val) in keys.iter_mut().enumerate() {
            if let Some(ind) = map.get(val) {
                if pos1 != *ind {
                    let pos2 = *ind as i32;
                    if pos2 != -1 {
                        num_vec[pos2 as usize] = num_vec[pos1];
                        num_vec[pos1] = *val;
                        break;
                    }
                } else {
                    -1 as i32;
                }
            } else {
                -1 as i32;
            };
        }

        num_vec
            .iter()
            .map(|x| (*x).to_string())
            .collect::<Vec<String>>()
            .join("")
            .as_str()
            .parse::<i32>()
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn it_works() {
        let mut expect = 7236;
        let mut result = Solution::maximum_swap(2736);
        assert_eq!(result, expect);

        expect = 9973;
        result = Solution::maximum_swap(9973);
        assert_eq!(result, expect);
    }
}
