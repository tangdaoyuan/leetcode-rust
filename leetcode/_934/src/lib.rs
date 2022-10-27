#![allow(dead_code)]
struct Solution;
impl Solution {
    pub fn shortest_bridge(_grid: Vec<Vec<i32>>) -> i32 {
        let mut grid = _grid.clone();
        let mut find = false;
        let mut edges = vec![];

        fn mark(row: i32, col: i32, grid: &mut Vec<Vec<i32>>, edges: &mut Vec<(i32, i32)>) {
            if !(row >= 0 && row < grid.len() as i32 && col >= 0 && col < grid[0].len() as i32)
                || grid[row as usize][col as usize] == 2
            {
                return;
            }
            if grid[row as usize][col as usize] == 0 {
                grid[row as usize][col as usize] = 2;
                edges.push((row as i32, col as i32));
                return;
            }
            grid[row as usize][col as usize] = 2;
            for direction in [(-1, 0), (1, 0), (0, 1), (0, -1)] {
                mark(row + direction.0, col + direction.1, grid, edges);
            }
        }

        let r_len = grid.len();
        let c_len = grid[0].len();
        for i in 0..r_len {
            for j in 0..c_len {
                if grid[i][j] == 1 {
                    mark(i as i32, j as i32, &mut grid, &mut edges);
                    find = true;
                    break;
                }

                if find {
                    break;
                }
            }

            if find {
                break;
            }
        }

        let mut result = 0;
        while !edges.is_empty() {
            let mut n = edges.len();

            result += 1;
            while n > 0 {
                n -= 1;
                let edge = edges.remove(0);
                for direction in [(-1, 0), (1, 0), (0, 1), (0, -1)] {
                    let nex = edge.0 + direction.0;
                    let ney = edge.1 + direction.1;
                    if (nex >= 0
                        && nex < grid.len() as i32
                        && ney >= 0
                        && ney < grid[0].len() as i32)
                        && grid[nex as usize][ney as usize] == 0
                    {
                        edges.push((nex, ney));
                        grid[nex as usize][ney as usize] = 2;
                    } else if (nex >= 0
                        && nex < grid.len() as i32
                        && ney >= 0
                        && ney < grid[0].len() as i32)
                        && grid[nex as usize][ney as usize] == 1
                    {
                        return result;
                    }
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut nums = vec![
            vec![1, 1, 1, 1, 1],
            vec![1, 0, 0, 0, 1],
            vec![1, 0, 1, 0, 1],
            vec![1, 0, 0, 0, 1],
            vec![1, 1, 1, 1, 1],
        ];
        let mut result = Solution::shortest_bridge(nums);
        assert_eq!(result, 1);
        nums = vec![vec![0, 1], vec![1, 0]];
        result = Solution::shortest_bridge(nums);
        assert_eq!(result, 1);
    }
}
