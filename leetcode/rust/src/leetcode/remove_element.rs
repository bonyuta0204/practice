pub struct Solution;

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        for i in (0..nums.len()).rev() {
            if nums[i] == val {
                nums.remove(i);
            }
        }
        return nums.len() as i32;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_element() {
        let mut vec = vec![3, 2, 2, 3];
        assert_eq!(Solution::remove_element(&mut vec, 3), 2);
        assert_eq!(vec, vec![2, 2]);
    }

    #[test]
    fn test_remove_duplicated_element() {
        let mut vec = vec![0, 1, 2, 2, 3, 0, 4, 2];
        assert_eq!(Solution::remove_element(&mut vec, 2), 3);
        assert_eq!(vec, vec![0, 1, 3, 0, 4]);
    }
}
