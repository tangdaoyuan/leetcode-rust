#![allow(dead_code)]

use std::collections::HashMap;
struct Solution;
impl Solution {
    pub fn custom_sort_string(order: String, s: String) -> String {
        let mut chars = HashMap::new();
        for ch in s.chars() {
            if !chars.contains_key(&ch) {
                chars.insert(ch, 0);
            }
            if let Some(val) = chars.get_mut(&ch) {
                *val = *val + 1;
            }
        }

        let mut ans = vec![];

        for o in order.chars() {
            let c = chars.get_mut(&o);
            if c.is_some() {
                let count = c.unwrap();
                if *count != 0 {
                    ans.push(o.to_string().repeat(*count));
                    *count = 0;
                }
            }
        }

        let mut tmp = chars.keys().collect::<Vec<_>>();
        tmp.sort();
        for o in tmp {
            let c = chars.get(o);
            if c.is_some() {
                let count = c.unwrap();
                if *count != 0 {
                    ans.push(o.to_string().repeat(*count));
                }
            }
        }

        ans.concat()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut order = String::from("cba");
        let mut s = String::from("abcd");
        let mut result = Solution::custom_sort_string(order, s);
        assert_eq!(result, String::from("cbad"));
        order = String::from("cbafg");
        s = String::from("abcd");
        result = Solution::custom_sort_string(order, s);
        assert_eq!(result, String::from("cbad"));
        order = String::from("hucw");
        s = String::from("utzoampdgkalexslxoqfkdjoczajxtuhqyxvlfatmptqdsochtdzgypsfkgqwbgqbcamdqnqztaqhqanirikahtmalzqjjxtqfnh");
        result = Solution::custom_sort_string(order, s);
        assert_eq!(result, String::from("hhhhhuucccwaaaaaaaaabbdddddeffffggggiijjjjkkkkllllmmmmnnnoooopppqqqqqqqqqqqrsssttttttttvxxxxxyyzzzzz"));
    }
}
