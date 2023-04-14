pub struct Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut result = String::new();

        let mut index = 0;
        loop {
            let c = match strs[0].chars().nth(index) {
                Some(v) => v,
                None => return result,
            };

            for i in 1..strs.len() {
                let alph = match strs[i].chars().nth(index) {
                    Some(v) => v,
                    None => return result,
                };

                if alph != c {
                    return  result;
                }
            }

            index += 1;
            result.push(c);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let strs = vec![
            String::from("flower"),
            String::from("flow"),
            String::from("flight"),
        ];

        assert_eq!(String::from("fl"), Solution::longest_common_prefix(strs));
    }

    #[test]
    fn test1() {
        let strs = vec![
            String::from("dog"),
            String::from("racecar"),
            String::from("car"),
        ];

        assert_eq!(String::from(""), Solution::longest_common_prefix(strs));
    }
}
