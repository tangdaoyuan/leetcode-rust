struct Solution;
impl Solution {
    pub fn image_smoother(img: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let directions: Vec<Vec<isize>> = vec![
            vec![-1, -1],
            vec![-1, 0],
            vec![0, 0],
            vec![0, -1],
            vec![1, 0],
            vec![0, 1],
            vec![1, 1],
            vec![-1, 1],
            vec![1, -1],
        ];
        let m = img.len() as isize;
        let n = img[0].len() as isize;

        let mut ans: Vec<Vec<i32>> = vec![vec![0; n as usize]; m as usize];

        println!("{:?}", ans);

        for i in 0..=m - 1 {
            for j in 0..=n - 1 {
                let mut count: i32 = 0;
                let mut sum: i32 = 0;
                for d in &directions {
                    if i + d[0] >= 0 && i + d[0] < m && j + d[1] >= 0 && j + d[1] < n {
                        count += 1;
                        sum += img[(i + d[0]) as usize][(j + d[1]) as usize];
                    }
                }
                ans[i as usize][j as usize] = sum / count;
            }
        }

        return ans;
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    #[test]
    fn image_smoother_works() {
        let img = vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]];
        let _ret = vec![vec![0, 0, 0], vec![0, 0, 0], vec![0, 0, 0]];
        let result = Solution::image_smoother(img);

        assert_eq!(result, _ret);
    }
}
