struct Solution;

impl Solution {
    pub fn reformat_number(number: String) -> String {
        let _number = number.replace("-", "").replace(" ", "");
        let len = _number.len();

        let mut count = _number.len();

        let mut prefix = "";
        let mut suffix = _number.as_str();

        if len > 4 {
            count = match len % 3 {
                2 => 2,
                c => c + 3,
            };
            prefix = _number.get(0..(len - count)).unwrap();
            suffix = _number.get((len - count)..).unwrap();
        }

        let mut charVec: Vec<char> = vec![];

        for i in prefix.chars().enumerate() {
            charVec.push(i.1);
            if (i.0 + 1) % 3 == 0 {
                charVec.push('-');
            }
        }

        if charVec.len() > 0 && count == 0 {
            charVec.pop();
        }

        let mut ret = String::from(charVec.iter().collect::<String>());
        ret.push_str(suffix);

        if count == 4 {
            ret.insert(ret.len() - 2, '-');
        }

        return ret;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut number = String::from("1-23-45 6");
        let mut result = Solution::reformat_number(number);
        assert_eq!(result, String::from("123-456"));
        number = String::from("123 4-567");
        result = Solution::reformat_number(number);
        assert_eq!(result, String::from("123-45-67"));
        number = String::from("123 4-5678");
        result = Solution::reformat_number(number);
        assert_eq!(result, String::from("123-456-78"));
        number = String::from("12");
        result = Solution::reformat_number(number);
        assert_eq!(result, String::from("12"));
        number = String::from("--17-5 229 35-39475 ");
        result = Solution::reformat_number(number);
        assert_eq!(result, String::from("175-229-353-94-75"));
    }
}
