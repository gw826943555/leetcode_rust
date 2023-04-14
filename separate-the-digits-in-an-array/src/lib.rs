pub struct Solution;

impl Solution {
    pub fn separate_digits(nums: Vec<i32>) -> Vec<i32> {
        let mut result = Vec::new();

        for num in nums {
            let num_c: Vec<char> = num.to_string().chars().collect();
            for i in 0..num_c.len() {
                result.push(num_c[i].to_digit(10).unwrap() as i32);
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let nums = vec![13, 25, 83, 77];
        let answer = vec![1,3,2,5,8,3,7,7];

        assert_eq!(answer, Solution::separate_digits(nums));
    }

    #[test]
    fn test1() {
        let nums = vec![7,1,3,9];
        let answer = vec![7,1,3,9];

        assert_eq!(answer, Solution::separate_digits(nums));
    }
}
