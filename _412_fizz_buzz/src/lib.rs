#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn fizz_buzz(n: i32) -> Vec<String> {
        (1..=n)
            .map(|i| {
                if i % 3 == 0 && i % 5 == 0 {
                    "FizzBuzz".to_string()
                } else if i % 3 == 0 {
                    "Fizz".to_string()
                } else if i % 5 == 0 {
                    "Buzz".to_string()
                } else {
                    i.to_string()
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::fizz_buzz(3),
            vec!["1".to_string(), "2".to_string(), "Fizz".to_string()]
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            Solution::fizz_buzz(5),
            vec![
                "1".to_string(),
                "2".to_string(),
                "Fizz".to_string(),
                "4".to_string(),
                "Buzz".to_string()
            ]
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            Solution::fizz_buzz(15),
            vec![
                "1".to_string(),
                "2".to_string(),
                "Fizz".to_string(),
                "4".to_string(),
                "Buzz".to_string(),
                "Fizz".to_string(),
                "7".to_string(),
                "8".to_string(),
                "Fizz".to_string(),
                "Buzz".to_string(),
                "11".to_string(),
                "Fizz".to_string(),
                "13".to_string(),
                "14".to_string(),
                "FizzBuzz".to_string()
            ]
        );
    }
}
