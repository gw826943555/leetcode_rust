use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn sort_by_bits(mut arr: Vec<i32>) -> Vec<i32> {
        arr.sort_unstable_by(|a, b| {a.count_ones().cmp(&b.count_ones()).then(a.cmp(b))});
        arr
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let arr = vec![0, 1, 2, 3, 4, 5, 6, 7, 8];
        let expected = vec![0, 1, 2, 4, 8, 3, 5, 6, 7];

        let result = Solution::sort_by_bits(arr);

        println!("{:?}", result);

        for i in 0..expected.len() {
            assert_eq!(expected[i], result[i]);
        }
    }

    #[test]
    fn test2() {
        let arr = vec![1024, 512, 256, 128, 64, 32, 16, 8, 4, 2, 1];
        let expected = vec![1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024];

        let result = Solution::sort_by_bits(arr);

        println!("{:?}", result);

        for i in 0..expected.len() {
            assert_eq!(expected[i], result[i]);
        }
    }

    #[test]
    fn test3() {
        let arr = vec![10000,10000];
        let expected = vec![10000,10000];

        let result = Solution::sort_by_bits(arr);

        println!("{:?}", result);

        for i in 0..expected.len() {
            assert_eq!(expected[i], result[i]);
        }
    }

    #[test]
    fn test4() {
        let arr = vec![2,3,5,7,11,13,17,19];
        let expected = vec![2,3,5,17,7,11,13,19];

        let result = Solution::sort_by_bits(arr);

        println!("{:?}", result);

        for i in 0..expected.len() {
            assert_eq!(expected[i], result[i]);
        }
    }

    #[test]
    fn test5() {
        let arr = vec![10,100,1000,10000];
        let expected = vec![10,100,10000,1000];

        let result = Solution::sort_by_bits(arr);

        println!("{:?}", result);

        for i in 0..expected.len() {
            assert_eq!(expected[i], result[i]);
        }
    }
}
