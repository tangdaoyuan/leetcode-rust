#![allow(dead_code)]
struct Solution;
impl Solution {
    pub fn parse_bool_expr(expression: String) -> bool {
        let mut stack = vec![];

        for ch in expression.chars() {
            if ch == ',' {
                continue;
            }
            if ch == ')' {
                let mut arr = vec![];
                while !stack.is_empty() && stack.last() != Some(&'(') {
                    arr.push(stack.pop().unwrap() == 't');
                }
                stack.pop();

                let op = stack.pop().unwrap();
                let ret = match op {
                    '&' => arr.iter().fold(true, |acc, cur| {
                        return acc && *cur;
                    }),
                    '|' => arr.iter().fold(false, |acc, cur| {
                        return acc || *cur;
                    }),
                    '!' => !(arr[0].clone()),
                    _ => false,
                };
                stack.push(if ret { 't' } else { 'f' });
            } else {
                stack.push(ch);
            }
        }
        stack.pop().unwrap() == 't'
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut expression = String::from("|(&(t,f,t),!(t))");
        let mut result = Solution::parse_bool_expr(expression);
        assert_eq!(result, false);
        expression = String::from("!(f)");
        result = Solution::parse_bool_expr(expression);
        assert_eq!(result, true);
        expression = String::from("|(f,t)");
        result = Solution::parse_bool_expr(expression);
        assert_eq!(result, true);
    }
}
