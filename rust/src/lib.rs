use std::collections::HashSet;

// https://leetcode.com/problems/optimal-partition-of-string/?envType=study-plan-v2&envId=amazon-spring-23-high-frequency
pub fn partition_string(s: String) -> i32 {
    let mut current_set = HashSet::new();

    let mut count = 0;

    for char in s.chars() {
        if current_set.contains(&char) {
            count = count + 1;
            current_set = HashSet::new();
            current_set.insert(char);
        } else {
            current_set.insert(char);
        }
    }

    count = count + 1;

    return count;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_partition_string() {
        assert_eq!(partition_string("abcabc".to_string()), 2);
    }
}
