#![allow(dead_code)]

struct Solution;
impl Solution {
    pub fn is_fliped_string(s1: String, s2: String) -> bool {
        let len = s1.len();
        if len != s2.len() {
            return false;
        }

        if len == 0 {
            return true;
        }
        for i in 0..len {
            let a = &s1[i..];
            let b = &s1[0..i];

            if s2.starts_with(a) && s2.ends_with(b) {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut s1 = String::from("waterbottle");
        let mut s2 = String::from("erbottlewat");
        let mut result = Solution::is_fliped_string(s1, s2);
        assert_eq!(result, true);
        s1 = String::from("aa");
        s2 = String::from("aba");
        result = Solution::is_fliped_string(s1, s2);
        assert_eq!(result, false);
    }
}
