#![allow(dead_code)]
struct Solution;
impl Solution {
    pub fn maximum_units(_box_types: Vec<Vec<i32>>, truck_size: i32) -> i32 {
        let mut box_types = _box_types.clone();
        box_types.sort_by(|a, b| b[1].cmp(&a[1]));

        let mut sum = 0;
        let mut rest = truck_size;
        for i in 0..box_types.len() {
            if rest >= box_types[i][0] {
                rest = rest - box_types[i][0];
                sum = sum + box_types[i][0] * box_types[i][1];
            } else {
                sum = sum + rest * box_types[i][1];
                break;
            }
        }
        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut box_types = vec![vec![5, 10], vec![2, 5], vec![4, 7], vec![3, 9]];
        let mut truct_size = 10;
        let mut result = Solution::maximum_units(box_types, truct_size);
        assert_eq!(result, 91);
        box_types = vec![vec![1, 3], vec![2, 2], vec![3, 1]];
        truct_size = 4;
        result = Solution::maximum_units(box_types, truct_size);
        assert_eq!(result, 8);
    }
}
