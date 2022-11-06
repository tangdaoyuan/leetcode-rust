#![allow(dead_code)]
struct Solution;
impl Solution {
    pub fn interpret(command: String) -> String {
        let mut ans = vec![];
        let mut ind = 0;
        while ind < command.len() {
            if &command[ind..ind + 1] == "G" {
                ans.push("G");
            } else if &command[ind..=ind + 1] == "()" {
                ans.push("o");
                ind += 1;
            } else {
                ans.push("al");
                ind += 3;
            }
            ind += 1;
        }

        ans.join("")
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut command = String::from("(al)G(al)()()G");
        let mut result = Solution::interpret(command);
        assert_eq!(result, String::from("alGalooG"));
        command = String::from("G()()()()(al)");
        result = Solution::interpret(command);
        assert_eq!(result, String::from("Gooooal"));
    }
}
