#![allow(dead_code)]

use std::{collections::HashMap};

struct Solution;
impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        if digits.len() == 0 {
            return vec![] as Vec<String>;
        }

        let phone_map: HashMap<_, _> = HashMap::from([
            ('2', "abc"),
            ('3', "def"),
            ('4', "ghi"),
            ('5', "jkl"),
            ('6', "mno"),
            ('7', "pqrs"),
            ('8', "tuv"),
            ('9', "wxyz"),
        ]);

        let mut combination: Vec<char> = vec![];
        let mut combinations: Vec<String> = vec![];

        fn back_trace(
            index: usize,
            digits: &String,
            phone_map: &HashMap<char, &str>,
            combination: &mut Vec<char>,
            combinations: &mut Vec<String>,
        ) {
            if index == digits.len() {
                combinations.push(combination.iter().collect::<String>());
            } else {
                let digit = digits.chars().nth(index).unwrap();
                let words = *(phone_map.get(&digit).unwrap());
                for letter in words.chars() {
                    combination.push(letter);
                    back_trace(index + 1, &digits, &phone_map, combination, combinations);
                    combination.pop();
                }
            }
        }

        back_trace(0, &digits, &phone_map, &mut combination, &mut combinations);
        combinations
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut digits = String::from("23");
        let mut result = Solution::letter_combinations(digits);
        assert_eq!(
            result,
            vec!["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]
                .iter()
                .map(|f| String::from(*f))
                .collect::<Vec<String>>()
        );
        digits = String::from("");
        result = Solution::letter_combinations(digits);
        assert_eq!(result, vec![] as Vec<String>);
        digits = String::from("2");
        result = Solution::letter_combinations(digits);
        assert_eq!(
            result,
            vec!["a", "b", "c"]
                .iter()
                .map(|f| String::from(*f))
                .collect::<Vec<String>>()
        );
    }
}
