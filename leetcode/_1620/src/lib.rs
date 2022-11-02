#![allow(dead_code)]
struct Solution;
impl Solution {
    pub fn best_coordinate(towers: Vec<Vec<i32>>, radius: i32) -> Vec<i32> {
        let mut max_x = i32::MIN;
        let mut max_y = i32::MIN;
        towers.iter().for_each(|tower| {
            let x = tower[0];
            let y = tower[1];
            max_x = max_x.max(x);
            max_y = max_y.max(y);
        });

        let mut ans = vec![0, 0, 0];
        for x in 0..=max_x {
            for y in 0..=max_y {
                let mut q = 0;
                towers.iter().for_each(|tower| {
                    let _x = tower[0] - x;
                    let _y = tower[1] - y;
                    let d = ((_x.pow(2) + _y.pow(2)) as f32).sqrt();

                    if d <= radius as f32 {
                        q += (tower[2] as f32 / (1.0 + d)).floor() as i32;
                    }
                });

                if q > ans[2] {
                    ans = vec![x, y, q];
                }
            }
        }
        ans[0..2].to_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let towers = vec![vec![1, 2, 5], vec![2, 1, 7], vec![3, 1, 9]];
        let radius = 2;
        let result = Solution::best_coordinate(towers, radius);
        assert_eq!(result, vec![2, 1]);
    }
}
