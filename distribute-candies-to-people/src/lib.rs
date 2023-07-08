struct Solution;

impl Solution {
    pub fn distribute_candies(candies: i32, num_people: i32) -> Vec<i32> {
        let mut res = vec![0; num_people as usize];
        if candies <= 0 {
            return res;
        }

        let n = (((1.0 + 8.0 * candies as f32).sqrt() - 1.0) / 2.0) as i32;
        let residual = candies - (n + n * (n - 1) / 2);
        let round = n / num_people;
        let round_residual = n % num_people;
        
        for i in 1..num_people + 1 {
            if i - 1 < round_residual
            {
                res[(i - 1) as usize] = (round + 1) * i + (round + 1) * round * num_people / 2;
            }
            else if i - 1 == round_residual {
                res[(i - 1) as usize] = round * i + round * (round - 1) * num_people / 2 + residual;
            }
            else {
                res[(i - 1) as usize] = round * i + round * (round - 1) * num_people / 2;
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = Solution::distribute_candies(7, 4);
        let expected = vec![1, 2, 3, 1];

        assert_eq!(result, expected);
    }

    #[test]
    fn test2() {
        let result = Solution::distribute_candies(10, 3);
        let expected = vec![5, 2, 3];

        assert_eq!(result, expected);
    }
}
