use std::collections::HashMap;

// list of pairs a^3 + b^3 = c^3 + d^3 where each is below than passed number n
// a, b, c, d >= 0
// n >= a >= b >= 0 AND n >= c >= d >= 0
// a + b >= c + d
fn main() {
    run_solution(BruteForce {}, 100);
    run_solution(BruteForce {}, 300);
    run_solution(TableLookup {}, 100);
    run_solution(TableLookup {}, 300);
    run_solution(TableLookup {}, 1000);
    run_solution(TableLookup {}, 3000);
}

fn run_solution(solution: impl Solution, n: u32) -> Vec<Pair> {
    let start = std::time::Instant::now();
    let results = solution.run(n);
    let duration = start.elapsed();
    println!(
        "Solution: {:12} took: {:8}ms, found: {:6} results",
        solution.name(),
        duration.as_millis(),
        results.len()
    );
    results
}

#[derive(Debug)]
#[allow(dead_code)]
struct Pair {
    a: u32,
    b: u32,
    c: u32,
    d: u32,
    value: u32,
}

impl Pair {
    #[allow(dead_code)]
    fn pretty_print(&self) {
        println!(
            "{:3}^3 + {:3}^3 = {:3}^3 + {:3}^3 = {}",
            self.a, self.b, self.c, self.d, self.value
        );
    }
}

trait Solution {
    fn name(&self) -> String;
    fn run(&self, n: u32) -> Vec<Pair>;
}

struct BruteForce;

impl Solution for BruteForce {
    fn name(&self) -> String {
        String::from("BruteForce")
    }

    fn run(&self, n: u32) -> Vec<Pair> {
        let mut results: Vec<Pair> = Vec::new();

        // n >= a >= b >= 0 AND n >= c >= d >= 0
        // a + b >= c + d

        for a in 0..n {
            for b in 0..a {
                for c in 0..n {
                    for d in 0..c {
                        if a == c && b == d {
                            continue;
                        }

                        if (a + b) >= (c + d) {
                            if a.pow(3) + b.pow(3) == c.pow(3) + d.pow(3) {
                                results.push(Pair {
                                    a,
                                    b,
                                    c,
                                    d,
                                    value: a.pow(3) + b.pow(3),
                                });
                            }
                        }
                    }
                }
            }
        }

        results
    }
}

struct TableLookup;

impl Solution for TableLookup {
    fn name(&self) -> String {
        String::from("TableLookup")
    }

    fn run(&self, n: u32) -> Vec<Pair> {
        let mut results: Vec<Pair> = Vec::new();
        let mut sum_map: HashMap<u32, Vec<(u32, u32)>> = HashMap::new();

        for a in 0..n {
            for b in 0..a {
                let sum = a.pow(3) + b.pow(3);
                sum_map.entry(sum).or_insert_with(Vec::new).push((a, b));
            }
        }

        for key in sum_map.keys() {
            let pairs = sum_map.get(key).unwrap();
            if pairs.len() > 1 {
                for i in 0..pairs.len() {
                    for j in 0..pairs.len() {
                        if i == j {
                            continue;
                        }
                        if (pairs[i].0 + pairs[i].1) >= (pairs[j].0 + pairs[j].1) {
                            results.push(Pair {
                                a: pairs[i].0,
                                b: pairs[i].1,
                                c: pairs[j].0,
                                d: pairs[j].1,
                                value: *key,
                            });
                        }
                    }
                }
            }
        }

        return results;
    }
}
