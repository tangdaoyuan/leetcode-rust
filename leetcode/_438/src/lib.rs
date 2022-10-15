#![allow(dead_code)]
#![allow(non_snake_case)]

use std::collections::HashMap;

struct Solution;
impl Solution {
    pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
        let k = p.len();
        let len = s.len();

        if len < k {
            return vec![];
        }
        let mut ans: Vec<i32> = vec![];

        let mut pMap: HashMap<char, i32> = HashMap::new();

        for ch in p.chars() {
            if pMap.get(&ch).is_none() {
                pMap.insert(ch, 0);
            }
            *pMap.get_mut(&ch).unwrap() = *pMap.get_mut(&ch).unwrap() + 1;
        }

        let mut sMap: HashMap<char, i32> = pMap.clone();

        for new_ch in s.get(0..k).unwrap().chars() {
            if pMap.get(&new_ch).is_some() {
                let target = sMap.get(&new_ch);
                if target.is_some() && target.unwrap() == &1 {
                    sMap.remove(&new_ch);
                } else {
                    sMap.insert(new_ch, *target.or(Some(&0)).unwrap() - 1);
                }
            }
        }

        if sMap.keys().len() == 0 {
            ans.push(0);
        }

        let _s: Vec<_> = s.chars().collect();

        for ind in 1..=len - k {
            let ch = _s[ind - 1];
            if pMap.get(&ch).is_some() {
                if sMap.get(&ch).or(Some(&0)).unwrap() == &-1 {
                    sMap.remove(&ch);
                } else {
                    sMap.insert(ch, *sMap.get(&ch).or(Some(&0)).unwrap() + 1);
                }
            }

            let new_ch = _s[ind + k - 1];
            if pMap.get(&new_ch).is_some() {
                let target = sMap.get(&new_ch);
                if target.is_some() && target.unwrap() == &1 {
                    sMap.remove(&new_ch);
                } else {
                    sMap.insert(new_ch, *target.or(Some(&0)).unwrap() - 1);
                }
            }

            if sMap.keys().len() == 0 {
                ans.push(ind as i32);
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
        let mut s = String::from("cbaebabacd");
        let mut p = String::from("abc");
        let mut result = Solution::find_anagrams(s, p);
        assert_eq!(result, vec![0, 6]);
        s = String::from("abab");
        p = String::from("ab");
        result = Solution::find_anagrams(s, p);
        assert_eq!(result, vec![0, 1, 2]);
        s = String::from("abacbabc");
        p = String::from("abc");
        result = Solution::find_anagrams(s, p);
        assert_eq!(result, vec![1, 2, 3, 5]);
    }
}
