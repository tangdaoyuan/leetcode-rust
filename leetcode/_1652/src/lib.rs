struct Solution;

impl Solution {
    pub fn decrypt(code: Vec<i32>, k: i32) -> Vec<i32> {
        let len = code.len();
        if k == 0 {
            return vec![0; len];
        }
        let mut ret: Vec<i32> = Vec::with_capacity(len);

        code.iter().enumerate().for_each(|(ind, _item)| {
            if k > 0 {
                if (ind + k as usize) >= len && ind + 1 < len {
                    let slice1 = &code[((ind + 1) % len)..];
                    let slice2 = &code[..=((ind + k as usize) % len)];
                    let s = [slice1, slice2].concat();
                    ret.push(s.iter().sum())
                } else {
                    let slice = &code[(ind + 1) % len..=(ind + k as usize) % len];
                    ret.push(slice.iter().sum());
                }
            } else {
                if (ind as i32 + k) < 0 && ind > 0 {
                    let slice1 = &code[..=((ind - 1) % len)];
                    let slice2 = &code[((len + ind) as i32 + k) as usize % len..];
                    let s = [slice1, slice2].concat();
                    ret.push(s.iter().sum())
                } else {
                    let slice =
                        &code[(((len + ind) as i32 + k) as usize) % len..=((len + ind) - 1) % len];
                    ret.push(slice.iter().sum());
                }
            }
        });

        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut code = vec![5, 7, 1, 4];
        let mut result = Solution::decrypt(code, 3);
        assert_eq!(result, vec![12, 10, 16, 13]);

        code = vec![1, 2, 3, 4];
        result = Solution::decrypt(code, 0);
        assert_eq!(result, vec![0, 0, 0, 0]);

        code = vec![2, 4, 9, 3];
        result = Solution::decrypt(code, -2);
        assert_eq!(result, vec![12, 5, 6, 13]);
    }
}
