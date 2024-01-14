struct Solution;

impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let len = nums.len();
        let mut left = 1;
        let mut right = len;
        let _mid = (left + right) / 2;

        // binary search
        while left <= right {
            let mid = (left + right) / 2;
            let max_sum = Solution::max_sum_for_length(&nums, mid);
            if max_sum >= target {
                if Solution::max_sum_for_length(&nums, mid - 1) < target {
                    return mid as i32;
                }
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        }

        return 0;
    }

    fn max_sum_for_length(nums: &Vec<i32>, length: usize) -> i32 {
        let mut max_sum = 0;
        let mut sum = 0;
        for i in 0..length {
            sum += nums[i];
        }
        max_sum = sum;

        for i in 0..(nums.len() - length) {
            sum -= nums[i];
            sum += nums[i + length];
            if sum > max_sum {
                max_sum = sum;
            }
        }

        return max_sum;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_sub_array_len() {
        assert_eq!(Solution::min_sub_array_len(7, vec![2, 3, 1, 2, 4, 3]), 2);
        assert_eq!(Solution::min_sub_array_len(4, vec![1, 4, 4]), 1);
        assert_eq!(
            Solution::min_sub_array_len(11, vec![1, 1, 1, 1, 1, 1, 1, 1]),
            0
        );
    }
}
