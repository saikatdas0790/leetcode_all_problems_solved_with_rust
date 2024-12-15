#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn number_of_steps(num: i32) -> i32 {
        std::iter::successors(Some(num), |&n| {
            if n == 0 {
                None
            } else if n % 2 == 0 {
                Some(n / 2)
            } else {
                Some(n - 1)
            }
        })
        .count() as i32
            - 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Solution::number_of_steps(14), 6);
        assert_eq!(Solution::number_of_steps(8), 4);
        assert_eq!(Solution::number_of_steps(123), 12);
    }
}
