fn main() {
    run_solution(
        SimpleSolution {},
        &Node::from_str("1"),
        &Node::from_str("9"),
    );
    run_solution(
        SimpleSolution {},
        &Node::from_str("123"),
        &Node::from_str("456"),
    );

    run_solution(
        SimpleSolution {},
        &Node::from_str("9999999999999"),
        &Node::from_str("1"),
    );

    run_solution(
        SimpleSolution {},
        &Node::from_str("123456789123456789123456789"),
        &Node::from_str("987654321987654321987654321"),
    );
    run_solution(SimpleSolution {}, &Node::from_int(0), &Node::from_int(0));
}

#[derive(Debug)]
struct Node {
    value: u32,
    next: Option<Box<Node>>,
}

fn run_solution(solution: impl Solution, node1: &Node, node2: &Node) {
    let start = std::time::Instant::now();
    let result = solution.solve(node1, node2);
    let duration = start.elapsed();
    println!(
        "Solution: {:12} took: {:8}ns, node1: {:10} , node2: {:10}, result: {:10}",
        solution.name(),
        duration.as_nanos(),
        node1.to_str(),
        node2.to_str(),
        result.to_str()
    );
}

impl Node {
    fn from_int(n: u32) -> Node {
        let next = n / 10;
        if next > 0 {
            Node {
                value: n % 10,
                next: Some(Box::new(Node::from_int(n / 10))),
            }
        } else {
            Node {
                value: n % 10,
                next: None,
            }
        }
    }

    fn from_str(str: &str) -> Node {
        let mut chars = str.chars();
        let value = chars.next().unwrap().to_digit(10).unwrap();
        let next = chars.as_str();
        if next.len() > 0 {
            Node {
                value: value,
                next: Some(Box::new(Node::from_str(next))),
            }
        } else {
            Node {
                value: value,
                next: None,
            }
        }
    }

    fn to_str(&self) -> String {
        format!(
            "{}{}",
            match &self.next {
                Some(node) => node.to_str(),
                None => String::from(""),
            },
            self.value
        )
    }
}

trait Solution {
    fn name(&self) -> String;
    fn solve(&self, node1: &Node, node2: &Node) -> Node;
}

struct SimpleSolution;

impl Solution for SimpleSolution {
    fn name(&self) -> String {
        String::from("SimpleSolution")
    }

    fn solve(&self, node1: &Node, node2: &Node) -> Node {
        fn solve_internal(node1: &Node, node2: &Node, carry: u32) -> Node {
            let sum = node1.value + node2.value + carry;
            let next_value = sum % 10;
            let carry = sum / 10;

            match &node1.next {
                Some(node1) => match &node2.next {
                    Some(node2) => Node {
                        value: next_value,
                        next: Some(Box::new(solve_internal(&node1, &node2, carry))),
                    },
                    None => Node {
                        value: next_value,
                        next: Some(Box::new(solve_internal(
                            &node1,
                            &Node {
                                value: 0,
                                next: None,
                            },
                            carry,
                        ))),
                    },
                },
                None => match &node2.next {
                    Some(node2) => Node {
                        value: next_value,
                        next: Some(Box::new(solve_internal(
                            &Node {
                                value: 0,
                                next: None,
                            },
                            &node2,
                            carry,
                        ))),
                    },
                    None => {
                        if carry > 0 {
                            Node {
                                value: next_value,
                                next: Some(Box::new(Node {
                                    value: carry,
                                    next: None,
                                })),
                            }
                        } else {
                            Node {
                                value: sum,
                                next: None,
                            }
                        }
                    }
                },
            }
        }

        solve_internal(node1, node2, 0)
    }
}
