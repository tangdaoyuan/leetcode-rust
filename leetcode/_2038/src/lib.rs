#![allow(dead_code)]

struct Solution;
impl Solution {
    pub fn winner_of_game(colors: String) -> bool {
        let len = colors.len();
        let mut freq = (0, 0);
        let char_vec: Vec<char> = colors.chars().collect();
        for (ind, _char) in char_vec.iter().enumerate() {
            if ind == 0 || ind == len - 1 {
                continue;
            }
            if char_vec[ind - 1] == *_char && char_vec[ind + 1] == *_char {
                if *_char == 'A' {
                    freq.0 = freq.0 + 1;
                } else {
                    freq.1 = freq.1 + 1;
                }
            }
        }
        return freq.0 > freq.1;
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn winner_of_game_works() {
        let colors = String::from("AAABABB");
        let result = Solution::winner_of_game(colors);
        assert_eq!(result, true);
    }
}
