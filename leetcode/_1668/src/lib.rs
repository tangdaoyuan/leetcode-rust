#![allow(dead_code)]
struct Solution;
impl Solution {
    pub fn max_repeating(sequence: String, word: String) -> i32 {
        let step = word.len();
        let len = sequence.len();
        let mut pos = 0;
        let mut k = 0;
        while pos < len {
            let mut count = 0;
            let mut _pos = pos;
            while _pos + step <= len && sequence[_pos.._pos + step] == word {
                _pos += step;
                count += 1;
            }
            k = k.max(count);
            pos += 1;
        }
        k
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut sequence = String::from("ababc");
        let mut word = String::from("ab");
        let mut result = Solution::max_repeating(sequence, word);
        assert_eq!(result, 2);
        sequence = String::from("ababc");
        word = String::from("ba");
        result = Solution::max_repeating(sequence, word);
        assert_eq!(result, 1);
        sequence = String::from("ababc");
        word = String::from("ac");
        result = Solution::max_repeating(sequence, word);
        assert_eq!(result, 0);
        sequence = String::from("aaabaaaabaaabaaaabaaaabaaaabaaaaba");
        word = String::from("aaaba");
        result = Solution::max_repeating(sequence, word);
        assert_eq!(result, 5);
    }
}
