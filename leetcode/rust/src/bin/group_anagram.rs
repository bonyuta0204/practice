use std::collections::HashMap;

fn main() {
    run_solution(
        CountSolution {},
        vec!["abc".to_string(), "bca".to_string(), "aaa".to_string()],
    );
}

fn run_solution(solution: impl Solution, strs: Vec<String>) {
    let start = std::time::Instant::now();
    let result = solution.solve(&strs);
    let duration = start.elapsed();
    println!(
        "Solution: {:20} took: {:8} nanosec strs: {:#?} result: {:#?}",
        solution.name(),
        duration.as_nanos(),
        strs,
        result
    );
}

trait Solution {
    fn name(&self) -> String;
    fn solve(&self, strs: &Vec<String>) -> Vec<Vec<String>>;
}

struct CountSolution;

impl Solution for CountSolution {
    fn name(&self) -> String {
        String::from("CountSolution")
    }

    fn solve(&self, strs: &Vec<String>) -> Vec<Vec<String>> {
        let mut maps = HashMap::new();

        for str in strs {
            let encoded = CountSolution::encode(str);

            let entry = maps.entry(encoded).or_insert(Vec::new());
            entry.push(str.clone())
        }

        maps.values().map(|v| v.clone()).collect()
    }
}

impl CountSolution {
    fn encode(str: &str) -> [u32; 26] {
        let mut encoded: [u32; 26] = [0; 26];

        let a = 'a' as usize;

        for c in str.chars() {
            let index = c as usize - a;
            encoded[index] += 1;
        }

        encoded
    }
}
