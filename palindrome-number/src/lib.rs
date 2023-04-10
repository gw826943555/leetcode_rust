pub struct Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        let mut num = x;
        let mut rev = 0;

        if x < 0 {
            return false;
        }

        while num > 0 {
            rev = rev * 10 + num % 10;
            num /= 10;
        }

        rev == x
    }
}

#[cfg(test)]
mod  tests {
    use crate::Solution;

    #[test]
    fn test121() {
        assert_eq!(Solution::is_palindrome(121), true);
    }

    #[test]
    fn testn121() {
        assert_eq!(Solution::is_palindrome(-121), false);
    }

    #[test]
    fn test10() {
        assert_eq!(Solution::is_palindrome(10), false);
    }

    #[test]
    fn test0() {
        assert_eq!(Solution::is_palindrome(0), true);
    }
}