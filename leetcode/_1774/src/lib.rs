#![allow(dead_code)]
struct Solution;
impl Solution {
    pub fn closest_cost(base_costs: Vec<i32>, topping_costs: Vec<i32>, target: i32) -> i32 {
        let mut ans = *base_costs.iter().min().unwrap();

        fn dfs(topping_costs: &Vec<i32>, p: usize, cur_cost: i32, target: i32, mut ans: &mut i32) {
            if (*ans - target).abs() < cur_cost - target {
                return;
            } else if (*ans - target).abs() >= (cur_cost - target).abs() {
                if (*ans - target).abs() > (cur_cost - target).abs() {
                    *ans = cur_cost;
                } else {
                    *ans = cur_cost.min(*ans);
                }
            }
            if p == topping_costs.len() {
                return;
            }
            dfs(
                topping_costs,
                p + 1,
                cur_cost + topping_costs[p] * 2,
                target,
                ans,
            );
            dfs(
                topping_costs,
                p + 1,
                cur_cost + topping_costs[p],
                target,
                ans,
            );
            dfs(topping_costs, p + 1, cur_cost, target, ans);
        }

        for val in base_costs {
            dfs(&topping_costs, 0, val, target, &mut ans);
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut result = Solution::closest_cost(vec![1, 7], vec![3, 4], 10);
        assert_eq!(result, 10);
        result = Solution::closest_cost(vec![2, 3], vec![4, 5, 100], 18);
        assert_eq!(result, 17);
        result = Solution::closest_cost(vec![3, 10], vec![2, 5], 9);
        assert_eq!(result, 8);
    }
}
