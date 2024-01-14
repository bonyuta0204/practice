use core::num;

struct Solution;

impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        for i in 0..(numbers.len() - 1) {
            for j in (i + 1)..numbers.len() {
                if numbers[i] + numbers[j] == target {
                    return vec![i as i32 + 1, j as i32 + 1];
                }
            }
        }

        return vec![];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_sum() {
        assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![1, 2]);
        assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 13), vec![1, 3]);
    }
}
