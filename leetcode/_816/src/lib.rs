#![allow(dead_code)]
struct Solution;
impl Solution {
    pub fn ambiguous_coordinates(_s: String) -> Vec<String> {
        let n = _s.len() - 2;
        let s = String::from(&_s[1..(_s.len() - 1)]);
        let mut ans = vec![];

        fn get_pos(s: &str) -> Vec<String> {
            let mut pos = vec![];
            if &s[0..1] != "0" || "0" == s {
                pos.push(String::from(s));
            }

            for p in 1..s.len() {
                if (p != 1 && &s[0..1] == "0") || (&s[s.len() - 1..] == "0") {
                    continue;
                }
                let a = format!("{}.{}", &s[0..p], &s[p..]);
                pos.push(a);
            }

            pos
        }

        for l in 1..n {
            let lt = get_pos(&s[0..l]);
            if lt.len() == 0 {
                continue;
            }
            let rt = get_pos(&s[l..]);
            if rt.len() == 0 {
                continue;
            }
            for i in lt.iter() {
                for j in rt.iter() {
                    ans.push(format!("({}, {})", i, j));
                }
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
        let mut s = String::from("(00011)");
        let result = Solution::ambiguous_coordinates(s);
        assert_eq!(
            result,
            vec![String::from("(0, 0.011)"), String::from("(0.001, 1)")]
        );
    }
}
