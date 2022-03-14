// 599 两个列表的最小索引总和

use std::collections::HashMap;

impl Solution {
    pub fn find_restaurant(list1: Vec<String>, list2: Vec<String>) -> Vec<String> {
        let mut map = HashMap::new();
        let mut ind = 0;
        let mut ret = vec![];
        let mut min = i64::MAX;
        
        for item in &list1 {
            if !map.contains_key(item) {
                map.insert(item, ind);
            }
            ind+=1;
        }

        ind = 0;
        for item in &list2 {
            if map.contains_key(item) {
                let val = match map.get(item) {
                    Some(&v) => v,
                    None => -1
                };
                if val + ind < min {
                    ret.clear();
                    min = val + ind;
                    ret.push(String::from(item));
                } else if val + ind == min {
                    ret.push(String::from(item));
                }

            }
            ind+=1;
        }
        return ret;
    }
}