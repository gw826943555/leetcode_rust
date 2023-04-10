pub struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        for m in 0..nums.len()
        {
            for n in (m + 1)..nums.len()
            {
                if nums[m] + nums[n] == target
                {
                    return vec![m as i32, n as i32];
                }
            }
        }

        vec![-1, -1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        let expected = vec![0, 1];

        let result = Solution::two_sum(nums, target);

        assert_eq!(expected.len(), result.len());
        for num in result {
            assert!(expected.contains(&num));
        }
    }

    #[test]
    fn test1() {
        let nums = vec![3, 2, 4];
        let target = 6;
        let expected = vec![1, 2];

        let result = Solution::two_sum(nums, target);

        assert_eq!(expected.len(), result.len());
        for num in result {
            assert!(expected.contains(&num));
        }
    }

    #[test]
    fn test2() {
        let nums = vec![3, 3];
        let target = 6;
        let expected = vec![0, 1];

        let result = Solution::two_sum(nums, target);

        assert_eq!(expected.len(), result.len());
        for num in result {
            assert!(expected.contains(&num));
        }
    }
}
