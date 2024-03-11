#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
        nums.iter().fold(Vec::new(), |mut acc, &x| {
            let last = *acc.last().unwrap_or(&0);
            acc.push(last + x);
            acc
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let nums = vec![1, 2, 3, 4];
        let expected = vec![1, 3, 6, 10];
        assert_eq!(Solution::running_sum(nums), expected);
    }

    #[test]
    fn test_2() {
        let nums = vec![1, 1, 1, 1, 1];
        let expected = vec![1, 2, 3, 4, 5];
        assert_eq!(Solution::running_sum(nums), expected);
    }

    #[test]
    fn test_3() {
        let nums = vec![3, 1, 2, 10, 1];
        let expected = vec![3, 4, 6, 16, 17];
        assert_eq!(Solution::running_sum(nums), expected);
    }
}
