#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
        accounts
            .iter()
            .map(|account| account.iter().sum())
            .max()
            .unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_maximum_wealth() {
        let accounts = vec![vec![1, 2, 3], vec![3, 2, 1]];
        assert_eq!(Solution::maximum_wealth(accounts), 6);
        let accounts = vec![vec![1, 5], vec![7, 3], vec![3, 5]];
        assert_eq!(Solution::maximum_wealth(accounts), 10);
        let accounts = vec![vec![2, 8, 7], vec![7, 1, 3], vec![1, 9, 5]];
        assert_eq!(Solution::maximum_wealth(accounts), 17);
    }
}
