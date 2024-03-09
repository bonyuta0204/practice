use std::collections::HashMap;

fn main() {
    run_solution(CountSolution {}, "abc", "bca");
    run_solution(CountSolution {}, "abc", "bbca");
    run_solution(CountSolution {}, "1234567890", "1234567890");
    run_solution(CountSolution {}, "1234567890", "2134567890");
    run_solution(CountSolution {}, "1234567890", "2134567891");
}

fn run_solution(solution: impl Solution, str1: &str, str2: &str) {
    let start = std::time::Instant::now();
    let result = solution.solve(str1, str2);
    let duration = start.elapsed();
    println!(
        "Solution: {:20} took: {:8} nanosec str1: {:30} str2: {:30} result: {}",
        solution.name(),
        duration.as_nanos(),
        str1,
        str2,
        result
    );
}

trait Solution {
    fn name(&self) -> String;
    fn solve(&self, str1: &str, str2: &str) -> bool;
}

struct CountSolution;

impl Solution for CountSolution {
    fn name(&self) -> String {
        String::from("CountSolution")
    }

    fn solve(&self, str1: &str, str2: &str) -> bool {
        let mut str1_map = HashMap::new();
        let mut str2_map = HashMap::new();

        if str1.len() != str2.len() {
            return false;
        }

        for char in str1.chars() {
            let count = str1_map.entry(char).or_insert(0);
            *count = *count + 1;
        }

        for char in str2.chars() {
            let count = str2_map.entry(char).or_insert(0);
            *count = *count + 1;
        }

        for char in str1_map.keys() {
            if str1_map.get(char) != str2_map.get(char) {
                return false;
            };
        }

        return true;
    }
}
