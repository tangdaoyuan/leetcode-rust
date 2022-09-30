#![allow(dead_code)]

use std::{collections::HashSet};

struct Solution;
impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let mut row_set: HashSet<usize> = HashSet::new();
        let mut col_set: HashSet<usize> = HashSet::new();
        for row in matrix.iter().enumerate() {
            for col in row.1.iter().enumerate() {
                if *(col.1) == 0 {
                    row_set.insert(row.0);
                    col_set.insert(col.0);
                }
            }
        }
        row_set.iter().for_each(|f| {
            matrix[*f].fill(0);
        });
        matrix.iter_mut().for_each(|r| {
            col_set.iter().for_each(|f| {
                r[*f] = 0;
            });
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_zeroes_works() {
        let mut result = vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]];
        Solution::set_zeroes(&mut result);
        let expect = vec![vec![1, 0, 1], vec![0, 0, 0], vec![1, 0, 1]];
        assert_eq!(result, expect);
        result = vec![vec![0, 1, 2, 0], vec![3, 4, 5, 2], vec![1, 3, 1, 5]];
        Solution::set_zeroes(&mut result);
        assert_eq!(
            result,
            vec![vec![0, 0, 0, 0], vec![0, 4, 5, 0], vec![0, 3, 1, 0]]
        );
    }
}
