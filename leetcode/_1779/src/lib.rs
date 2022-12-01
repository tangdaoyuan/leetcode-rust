#![allow(dead_code)]
struct Solution;
impl Solution {
    pub fn nearest_valid_point(x: i32, y: i32, points: Vec<Vec<i32>>) -> i32 {
        let row = points
            .iter()
            .enumerate()
            .filter(|(_pos, point)| point[0] == x)
            .min_by(|(_pos1, a), (_pos2, b)| (a[1] - y).abs().cmp(&(b[1] - y).abs()))
            .map_or((-1, i32::MAX), |v| {
                (v.0 as i32, (v.1[0] - x).abs() + (v.1[1] - y).abs())
            });

        let col = points
            .iter()
            .enumerate()
            .filter(|(_pos, point)| point[1] == y)
            .min_by(|(_pos1, a), (_pos2, b)| (a[0] - x).abs().cmp(&(b[0] - x).abs()))
            .map_or((-1, i32::MAX), |v| {
                (v.0 as i32, (v.1[0] - x).abs() + (v.1[1] - y).abs())
            });
        if row.1 < col.1 {
            row.0
        } else if row.1 > col.1 {
            col.0
        } else {
            row.0.min(col.0)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut result = Solution::nearest_valid_point(
            3,
            4,
            vec![vec![1, 2], vec![3, 1], vec![2, 4], vec![2, 3], vec![4, 4]],
        );
        assert_eq!(result, 2);
        result = Solution::nearest_valid_point(3, 4, vec![vec![3, 4]]);
        assert_eq!(result, 0);
        result = Solution::nearest_valid_point(3, 4, vec![vec![2, 3]]);
        assert_eq!(result, -1);
    }
}
