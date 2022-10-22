#![allow(dead_code)]

struct Solution;
impl Solution {
    pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
        let r_len = matrix.len();
        if r_len == 0 {
            return 0;
        }
        let c_len = matrix[0].len();

        let mut rec: Vec<Vec<i32>> = matrix
            .iter()
            .map(|f| -> Vec<i32> {
                f.iter()
                    .map(|c| -> i32 { c.to_digit(10).unwrap() as i32 })
                    .collect()
            })
            .collect();

        for i in 1..r_len {
            for j in 0..c_len {
                if rec[i][j] == 1 {
                    rec[i][j] += rec[i - 1][j];
                }
            }
        }
        let mut stack;
        let mut max_val = 0;

        let mut count = 0;

        for row in rec {
            stack = vec![];
            let mut dp = vec![0; row.len()];
            row.iter().enumerate().for_each(|(ind, val)| {
                let mut i = ind;
                let mut same = false;

                while !stack.is_empty() && *val <= row[*stack.last().unwrap()] {
                    if row[ind] == row[*stack.last().unwrap()] {
                        i = dp[*stack.last().unwrap()];
                        same = true;
                        break;
                    }
                    i = dp[stack.pop().unwrap()];
                }
                if stack.is_empty() {
                    i = 0;
                }

                if !same {
                    stack.push(ind);
                }

                dp[ind] = i;
            });
            stack = vec![];

            let mut rdp = vec![0; row.len()];
            for ind in (0..c_len).rev() {
                let mut i = ind;
                let mut same = false;

                while !stack.is_empty() && row[ind] <= row[*stack.last().unwrap()] {
                    if row[ind] == row[*stack.last().unwrap()] {
                        i = rdp[*stack.last().unwrap()];
                        same = true;
                        break;
                    }
                    i = rdp[stack.pop().unwrap()];
                }

                if stack.is_empty() {
                    i = c_len - 1;
                }

                if !same {
                    stack.push(ind);
                }

                rdp[ind] = i;
                max_val = max_val.max((i - dp[ind] + 1) as i32 * row[ind]);
            }

            count += 1;
        }
        max_val
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut matrix = vec![
            vec!['1', '0', '1', '0', '0'],
            vec!['1', '0', '1', '1', '1'],
            vec!['1', '1', '1', '1', '1'],
            vec!['1', '0', '0', '1', '0'],
        ];
        let result = Solution::maximal_rectangle(matrix);
        assert_eq!(result, 6);
        matrix = vec![];
        assert_eq!(Solution::maximal_rectangle(matrix), 0);
        matrix = vec![vec!['0']];
        assert_eq!(Solution::maximal_rectangle(matrix), 0);
        matrix = vec![vec!['1']];
        assert_eq!(Solution::maximal_rectangle(matrix), 1);
        matrix = vec![vec!['0', '0']];
        assert_eq!(Solution::maximal_rectangle(matrix), 0);
        matrix = vec![vec!['1', '0']];
        assert_eq!(Solution::maximal_rectangle(matrix), 1);
        matrix = vec![
            vec!['1', '0', '0', '0', '1'],
            vec!['1', '1', '0', '1', '1'],
            vec!['1', '1', '1', '1', '1'],
        ];
        assert_eq!(Solution::maximal_rectangle(matrix), 5);

        matrix = vec![
            vec!['0', '1', '1', '0', '0', '1', '0', '1', '0', '1'],
            vec!['0', '0', '1', '0', '1', '0', '1', '0', '1', '0'],
            vec!['1', '0', '0', '0', '0', '1', '0', '1', '1', '0'],
            vec!['0', '1', '1', '1', '1', '1', '1', '0', '1', '0'],
            vec!['0', '0', '1', '1', '1', '1', '1', '1', '1', '0'],
            vec!['1', '1', '0', '1', '0', '1', '1', '1', '1', '0'],
            vec!['0', '0', '0', '1', '1', '0', '0', '0', '1', '0'],
            vec!['1', '1', '0', '1', '1', '0', '0', '1', '1', '1'],
            vec!['0', '1', '0', '1', '1', '0', '1', '0', '1', '1'],
        ];
        assert_eq!(Solution::maximal_rectangle(matrix), 10);

        matrix = vec![
            vec!['0', '0', '0', '1', '0', '1', '1', '1'],
            vec!['0', '1', '1', '0', '0', '1', '0', '1'],
            vec!['1', '0', '1', '1', '1', '1', '0', '1'],
            vec!['0', '0', '0', '1', '0', '0', '0', '0'],
            vec!['0', '0', '1', '0', '0', '0', '1', '0'],
            vec!['1', '1', '1', '0', '0', '1', '1', '1'],
            vec!['1', '0', '0', '1', '1', '0', '0', '1'],
            vec!['0', '1', '0', '0', '1', '1', '0', '0'],
            vec!['1', '0', '0', '1', '0', '0', '0', '0'],
        ];
        assert_eq!(Solution::maximal_rectangle(matrix), 4);

        matrix = vec![
            vec![
                '0', '1', '1', '0', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1',
                '1', '1', '0',
            ],
            vec![
                '1', '0', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1',
                '1', '1', '1',
            ],
            vec![
                '1', '1', '0', '1', '1', '1', '1', '1', '1', '1', '1', '1', '0', '1', '1', '1',
                '1', '1', '1',
            ],
            vec![
                '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '0', '1', '1',
                '1', '1', '1',
            ],
            vec![
                '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '0', '1',
                '1', '1', '1',
            ],
            vec![
                '1', '1', '1', '0', '1', '1', '1', '0', '1', '1', '1', '1', '1', '1', '1', '1',
                '1', '0', '1',
            ],
            vec![
                '1', '0', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '0', '1',
                '1', '1', '1',
            ],
            vec![
                '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '0',
                '1', '1', '0',
            ],
            vec![
                '0', '0', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '0',
                '1', '1', '1',
            ],
            vec![
                '1', '1', '0', '1', '1', '1', '1', '1', '1', '1', '0', '1', '1', '1', '1', '1',
                '1', '1', '1',
            ],
            vec![
                '1', '1', '1', '1', '1', '1', '1', '1', '1', '0', '1', '1', '1', '1', '1', '1',
                '1', '1', '1',
            ],
            vec![
                '0', '1', '1', '0', '1', '1', '1', '0', '1', '1', '1', '1', '1', '1', '1', '1',
                '1', '1', '1',
            ],
            vec![
                '1', '1', '1', '1', '0', '1', '1', '1', '1', '1', '1', '1', '1', '1', '0', '1',
                '1', '1', '1',
            ],
            vec![
                '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1',
                '1', '1', '1',
            ],
            vec![
                '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1',
                '1', '1', '1',
            ],
            vec![
                '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1',
                '1', '0', '1',
            ],
            vec![
                '1', '1', '1', '1', '1', '1', '1', '1', '0', '1', '1', '0', '1', '1', '0', '1',
                '1', '1', '1',
            ],
            vec![
                '1', '1', '1', '1', '1', '1', '0', '1', '1', '1', '1', '1', '1', '1', '1', '0',
                '1', '1', '1',
            ],
        ];
        assert_eq!(Solution::maximal_rectangle(matrix), 51);
    }
}
