#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn can_form_array(arr: Vec<i32>, pieces: Vec<Vec<i32>>) -> bool {
        let mut i = 0;
        let len = arr.len();
        while i < len {
            let val = arr.get(i).unwrap();
            let select: Vec<_> = pieces
                .iter()
                .filter(|i| *((*i).get(0).unwrap()) == *val)
                .collect();
            if select.len() == 0 {
                return false;
            }
            let temp = *(select.get(0).unwrap());

            for c in temp.iter() {
                if *c == *(arr.get(i).unwrap()) {
                    i += 1;
                } else {
                    return false;
                }
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut arr = vec![15, 88];
        let mut pieces = vec![vec![88], vec![15]];
        assert_eq!(Solution::can_form_array(arr, pieces), true);

        arr = vec![91, 4, 64, 78];
        pieces = vec![vec![78], vec![4, 64], vec![91]];
        assert_eq!(Solution::can_form_array(arr, pieces), true);

        arr = vec![49, 18, 16];
        pieces = vec![vec![16, 18, 49]];
        assert_eq!(Solution::can_form_array(arr, pieces), false);
    }
}
