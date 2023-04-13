use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let digits = HashMap::from([
            ('I', 1),
            ('V', 5),
            ('X', 10),
            ('L', 50),
            ('C', 100),
            ('D', 500),
            ('M', 1000),
        ]);

        let chars : Vec<_> = s.chars().collect();
        let mut result = *digits.get(&chars[chars.len() - 1]).unwrap();

        for i in 0..chars.len() - 1
        {
            let cur  = digits.get(&chars[i]).unwrap();
            let next = digits.get(&chars[i + 1]).unwrap();

            if cur >= next {
                result += cur;
            }
            else {
                result -= cur;
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3() {
        let result = Solution::roman_to_int("III".to_string());
        assert_eq!(result, 3);
    }

    #[test]
    fn test_4() {
        let result = Solution::roman_to_int("IV".to_string());
        assert_eq!(result, 4);
    }

    #[test]
    fn test_9() {
        let result = Solution::roman_to_int("IX".to_string());
        assert_eq!(result, 9);
    }

    #[test]
    fn test_58() {
        let result = Solution::roman_to_int("LVIII".to_string());
        assert_eq!(result, 58);
    }

    #[test]
    fn test_1994() {
        let result = Solution::roman_to_int("MCMXCIV".to_string());
        assert_eq!(result, 1994);
    }
}
