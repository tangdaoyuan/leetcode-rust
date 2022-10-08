#![allow(dead_code)]

struct Solution;
impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut ret: Vec<String> = vec![];
        let mut stack: Vec<char> = vec![];

        fn back_track(stack: &mut Vec<char>, left: i32, right: i32, n: i32, ret: &mut Vec<String>) {
            if stack.len() == 2 * (n as usize) {
                ret.push(stack.iter().collect::<String>());
                return;
            }
            if left < n {
                stack.push('(');
                back_track(stack, left + 1, right, n, ret);
                stack.pop();
            }
            if right < left {
                stack.push(')');
                back_track(stack, left, right + 1, n, ret);
                stack.pop();
            }
        }
        back_track(&mut stack, 0, 0, n, &mut ret);
        ret
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let n = 3;
        let result = Solution::generate_parenthesis(n);
        assert_eq!(
            result,
            ["((()))", "(()())", "(())()", "()(())", "()()()"]
                .iter()
                .map(|f| String::from(*f))
                .collect::<Vec<String>>()
        );
    }
}
