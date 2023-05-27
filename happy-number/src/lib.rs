use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn is_happy(n: i32) -> bool {
        let mut sumset = HashSet::new();
        let mut n_str = n.to_string();

        loop {
            let mut sum = 0;

            for digit in n_str.chars() {
                sum += digit.to_digit(10).unwrap().pow(2) as i32;
            }

            if sum == 1 {
                return true;
            }
            else {
                if sumset.contains(&sum) {
                    return false;
                }
                else {
                    sumset.insert(sum);
                    n_str = sum.to_string();
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn happy_19() {
        assert_eq!(Solution::is_happy(19), true)
    }

    #[test]
    fn happy_2() {
        assert_eq!(Solution::is_happy(2), false)
    }
}
