#![allow(dead_code)]

struct Solution;
impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let mut dp: Vec<usize> = vec![0; s.len()];
        let mut max_val = 0;
        let chars = s.chars();
        for (ind, ch) in chars.enumerate() {
            if ind == 0 {
                continue;
            }
            if ch == ')' {
                if s.get(ind - 1..ind).unwrap() == "(" {
                    dp[ind] = 2;

                    if ind >= 2 {
                        dp[ind] = dp[ind] + dp[ind - 2];
                    }
                } else if dp[ind - 1] > 0 {
                    if ind - dp[ind - 1] >= 1
                        && s.get(ind - dp[ind - 1] - 1..ind - dp[ind - 1]).unwrap() == "("
                    {
                        dp[ind] = dp[ind - 1] + 2;
                        if ind - dp[ind - 1] >= 2 {
                            dp[ind] += dp[ind - dp[ind - 1] - 2];
                        }
                    }
                }
            }
            max_val = max_val.max(*dp.get(ind).unwrap());
        }
        max_val as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut s = String::from("(()");
        let mut result = Solution::longest_valid_parentheses(s);
        assert_eq!(result, 2);
        s = String::from(")()())");
        result = Solution::longest_valid_parentheses(s);
        assert_eq!(result, 4);
        s = String::from("");
        result = Solution::longest_valid_parentheses(s);
        assert_eq!(result, 0);
    }
}
