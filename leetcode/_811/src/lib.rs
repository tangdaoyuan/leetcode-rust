#![allow(dead_code)]

use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn subdomain_visits(cpdomains: Vec<String>) -> Vec<String> {
        let mut map: HashMap<String, i32> = HashMap::new();

        for cpdomain in cpdomains {
            let content = cpdomain.split(" ").collect::<Vec<_>>();
            let count = (*(content.get(0).unwrap())).parse::<i32>().unwrap();
            let domain = *(content.get(1).unwrap());
            let indexes: Vec<_> = domain.match_indices(".").map(|(i, _)| i).collect();

            let a = map.get(domain).or(Some(&0)).unwrap();
            map.insert(domain.to_string(), count + *a);

            for i in indexes {
                let key = domain.get(i + 1..).unwrap();
                let a = map.get(key).or(Some(&0)).unwrap();
                map.insert(key.to_string(), count + *a);
            }
        }

        map.into_iter()
            .map(|entry| String::from(format!("{} {}", entry.1, entry.0)))
            .collect::<Vec<_>>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut cpdomains = vec![String::from("9001 discuss.leetcode.com")];
        let mut result = Solution::subdomain_visits(cpdomains);
        result.sort();
        assert_eq!(
            result,
            vec![
                String::from("9001 com"),
                String::from("9001 discuss.leetcode.com"),
                String::from("9001 leetcode.com"),
            ]
        );
        cpdomains = vec![
            String::from("900 google.mail.com"),
            String::from("50 yahoo.com"),
            String::from("1 intel.mail.com"),
            String::from("5 wiki.org"),
        ];
        result = Solution::subdomain_visits(cpdomains);
        result.sort();
        assert_eq!(
            result,
            vec![
                String::from("1 intel.mail.com"),
                String::from("5 org"),
                String::from("5 wiki.org"),
                String::from("50 yahoo.com"),
                String::from("900 google.mail.com"),
                String::from("901 mail.com"),
                String::from("951 com"),
            ]
        );
    }
}
