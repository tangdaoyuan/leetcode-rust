#![allow(dead_code)]
struct Solution;
impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let mut visited = vec![0; num_courses as usize];
        let mut edges: Vec<Vec<usize>> = vec![vec![]; num_courses as usize];
        let mut valid = true;

        for i in prerequisites {
            edges[i[1] as usize].push(i[0] as usize);
        }

        fn dfs(pos: usize, visited: &mut Vec<i32>, edges: &Vec<Vec<usize>>) -> bool {
            visited[pos] = 1;
            for v in &edges[pos] {
                if visited[*v] == 0 {
                    if !dfs(*v, visited, edges) {
                        return false;
                    }
                } else if visited[*v] == 1 {
                    return false;
                }
            }
            true
        }

        for i in 0..num_courses {
            if valid && visited[i as usize] == 0 {
                valid = dfs(i as usize, &mut visited, &edges);
            }
        }

        valid
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut num_courses = 2;
        let mut prerequisites = vec![vec![1, 0]];
        let mut result = Solution::can_finish(num_courses, prerequisites);
        assert_eq!(result, true);
        num_courses = 2;
        prerequisites = vec![vec![1, 0], vec![0, 1]];
        result = Solution::can_finish(num_courses, prerequisites);
        assert_eq!(result, false);
    }
}
