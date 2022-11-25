#![allow(dead_code)]
struct Solution;
impl Solution {
    pub fn expressive_words(s: String, words: Vec<String>) -> i32 {
        let mut dp = vec![];

        for ch in s.chars() {
            match dp.last_mut() {
                Some((cha, count)) => {
                    if *cha == ch {
                        *count += 1;
                    } else {
                        dp.push((ch, 1));
                    }
                }
                None => dp.push((ch, 1)),
            }
        }

        let mut ans = 0;
        for s in words {
            let mut pos = 0;
            let mut p = 0;
            let len = s.len();
            while p < len && pos < dp.len() {
                let mut count = 0;
                while pos < dp.len() && p < len && dp[pos].0 == s.chars().nth(p).unwrap() {
                    count += 1;
                    p += 1;
                }

                if count == dp[pos].1 || (dp[pos].1 >= 3 && count > 0 && count <= dp[pos].1) {
                    pos += 1;
                    continue;
                } else {
                    break;
                }
            }
            if p == len && pos == dp.len() {
                ans += 1;
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut s = String::from("heeellooo");
        let mut words = vec![
            String::from("hello"),
            String::from("hi"),
            String::from("helo"),
        ];
        assert_eq!(Solution::expressive_words(s, words), 1);

        s = String::from("zzzzzyyyyy");
        words = vec![
            String::from("zzyy"),
            String::from("zy"),
            String::from("zyy"),
        ];
        assert_eq!(Solution::expressive_words(s, words), 3);
        s = String::from("abcd");
        words = vec![String::from("abc")];
        assert_eq!(Solution::expressive_words(s, words), 0);
    }
}
