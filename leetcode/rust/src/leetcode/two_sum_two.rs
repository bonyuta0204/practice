pub struct Solution;

impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let largest = numbers[numbers.len() - 1];

        for i in 0..(numbers.len() - 1) {
            let left = numbers[i];

            if left + largest < target {
                continue;
            }

            for j in (i + 1)..numbers.len() {
                let right = numbers[j];
                if left + right == target {
                    return vec![i as i32 + 1, j as i32 + 1];
                }
            }
        }

        vec![]
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
