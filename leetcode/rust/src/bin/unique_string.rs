use std::collections::{HashMap, HashSet};

fn main() {
    run_solution(PlainSolution {}, "abc");
    run_solution(PlainSolution {}, "abcdefghijklmnopqrstu");
    run_solution(PlainSolution {}, "abcdefgab");

    run_solution(SetSolution {}, "abc");
    run_solution(SetSolution {}, "abcdefghijklmnopqrstu");
    run_solution(SetSolution {}, "abcdefgab");
}

fn run_solution(solution: impl Solution, string: &str) {
    let start = std::time::Instant::now();
    let result = solution.solve(string);
    let duration = start.elapsed();
    println!(
        "Solution: {:20} took: {:8} nanosec string: {:30}result: {}",
        solution.name(),
        duration.as_nanos(),
        string,
        result
    );
}

trait Solution {
    fn name(&self) -> String;
    fn solve(&self, string: &str) -> bool;
}

struct PlainSolution;
struct SetSolution;

impl Solution for PlainSolution {
    fn name(&self) -> String {
        String::from("PlainSolution")
    }

    fn solve(&self, string: &str) -> bool {
        let mut table = HashMap::new();
        for char in string.chars() {
            if let Some(_) = table.get(&char) {
                return false;
            }
            table.insert(char, true);
        }
        true
    }
}

impl Solution for SetSolution {
    fn name(&self) -> String {
        String::from("SetSolution")
    }

    fn solve(&self, string: &str) -> bool {
        let mut table = HashSet::new();
        for char in string.chars() {
            if !table.insert(char) {
                return false;
            }
        }
        true
    }
}
