#![allow(dead_code)]
#![allow(non_snake_case)]
use std::collections::HashMap;
struct Solution;
impl Solution {
    pub fn shortest_path_all_keys(grid: Vec<String>) -> i32 {
        let directions: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        let m = grid.len();
        let n = grid[0].len();
        let mut sx = 0;
        let mut sy = 0;
        let mut keyToIndex: HashMap<_, _> = HashMap::new();
        for i in 0..m {
            for j in 0..n {
                let ch = grid[i].chars().nth(j).unwrap();
                if ch == '@' {
                    sx = i;
                    sy = j;
                } else if ch >= 'a' && ch <= 'z' {
                    if !keyToIndex.contains_key(&ch) {
                        let idx = keyToIndex.len();
                        keyToIndex.insert(ch, idx as i32);
                    }
                }
            }
        }

        // bfs
        let mut queue = vec![];
        let mut dp = vec![vec![vec![-1; 1 << keyToIndex.len()]; n]; m];
        queue.push((sx, sy, 0));
        dp[sx][sy][0] = 0;
        while !queue.is_empty() {
            let arr = queue.remove(0);
            let x = arr.0;
            let y = arr.1;
            let mask = arr.2;
            for i in directions {
                if (x as i32 + i.0 < 0) || (y as i32 + i.1 < 0) {
                    continue;
                }
                let nx = (x as i32 + i.0) as usize;
                let ny = (y as i32 + i.1) as usize;

                if nx < m && ny < n && grid[nx].get(ny..(ny + 1)).unwrap() != "#" {
                    let ch = grid[nx].chars().nth(ny).unwrap();

                    if ch == '.' || ch == '@' {
                        if dp[nx][ny][mask] == -1 {
                            dp[nx][ny][mask] = dp[x][y][mask] + 1;
                            queue.push((nx, ny, mask));
                        }
                    } else if ch >= 'a' && ch <= 'z' {
                        let idx = *keyToIndex.get(&ch).unwrap();
                        if dp[nx][ny][mask | (1 << idx)] == -1 {
                            dp[nx][ny][mask | (1 << idx)] = dp[x][y][mask] + 1;

                            if (mask | (1 << idx)) == (1 << keyToIndex.len()) - 1 {
                                return dp[nx][ny][mask | (1 << idx)];
                            }
                            queue.push((nx, ny, mask | (1 << idx)));
                        }
                    } else {
                        let idx = *keyToIndex.get(&ch.to_ascii_lowercase()).unwrap();
                        if ((mask & (1 << idx)) != 0) && (dp[nx][ny][mask] == -1) {
                            dp[nx][ny][mask] = dp[x][y][mask] + 1;
                            queue.push((nx, ny, mask));
                        }
                    }
                }
            }
        }
        -1
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let grid = vec![
            String::from("@.a.#"),
            String::from("###.#"),
            String::from("b.A.B"),
        ];
        let result = Solution::shortest_path_all_keys(grid);
        assert_eq!(result, 8);
    }
}
