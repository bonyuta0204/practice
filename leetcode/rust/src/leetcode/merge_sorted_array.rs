pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    let mut last_insert_index = 0;
    let mut current_length = m;
    for n2 in nums2 {
        for array_i in last_insert_index..(m + n) {
            let n1 = nums1[array_i as usize];

            if n1 > *n2 {
                nums1.insert(array_i as usize, *n2);
                nums1.pop();

                last_insert_index = array_i;
                current_length = current_length + 1;
                break;
            } else if current_length == array_i {
                nums1.insert(array_i as usize, *n2);
                nums1.pop();

                last_insert_index = array_i + 1;
                current_length = current_length + 1;
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge() {
        let mut nums1 = vec![1, 2, 3, 0, 0, 0];
        let mut nums2 = vec![2, 5, 6];
        merge(&mut nums1, 3, &mut nums2, 3);
        assert_eq!(nums1, vec![1, 2, 2, 3, 5, 6]);
    }
}
