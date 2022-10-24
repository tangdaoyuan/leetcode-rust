#![allow(dead_code)]
struct Solution;
impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let row_len = matrix.len();
        if row_len == 0 {
            return;
        }
        let col_len = matrix[0].len();
        let mid = (row_len - 1) / 2;

        for r in 0..=mid {
            for c in 0..col_len {
                let temp = matrix[r][c];
                matrix[r][c] = matrix[row_len - 1 - r][c];
                matrix[row_len - 1 - r][c] = temp;
            }
        }
        for r in 0..row_len {
            for c in 0..col_len {
                if r < c {
                    let temp = matrix[r][c];
                    matrix[r][c] = matrix[c][r];
                    matrix[c][r] = temp;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        Solution::rotate(&mut matrix);
        assert_eq!(matrix, vec![vec![7, 4, 1], vec![8, 5, 2], vec![9, 6, 3]]);
    }
}
