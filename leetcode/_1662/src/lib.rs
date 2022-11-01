#![allow(dead_code)]
struct Solution;
impl Solution {
    pub fn array_strings_are_equal(word1: Vec<String>, word2: Vec<String>) -> bool {
        let str1: String = word1.concat();
        let str2: String = word2.concat();
        str1 == str2
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let word1 = vec![String::from("ab"), String::from("c")];
        let word2 = vec![String::from("a"), String::from("bc")];
        let result = Solution::array_strings_are_equal(word1, word2);
        assert_eq!(result, true);
    }
}
