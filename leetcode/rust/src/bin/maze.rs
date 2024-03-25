use std::collections::VecDeque;

fn main() {
    let maze1 = Maze {
        data: vec![
            vec![
                Cell::Space,
                Cell::Wall,
                Cell::Space,
                Cell::Space,
                Cell::Space,
            ],
            vec![
                Cell::Space,
                Cell::Space,
                Cell::Space,
                Cell::Wall,
                Cell::Space,
            ],
            vec![Cell::Wall, Cell::Wall, Cell::Space, Cell::Wall, Cell::Space],
            vec![
                Cell::Space,
                Cell::Wall,
                Cell::Space,
                Cell::Space,
                Cell::Space,
            ],
            vec![Cell::Space, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Space],
        ],
    };

    let start = Point { x: 0, y: 0 };
    let end = Point { x: 4, y: 4 };

    run_solution(BFSSolution {}, &maze1, &start, &end);
}

#[derive(Debug, Clone, Copy)]
enum Cell {
    Wall,
    Space,
}

#[derive(Debug, Clone)]
struct Maze {
    data: Vec<Vec<Cell>>,
}

impl Maze {
    fn get_value(&self, point: &Point) -> Cell {
        self.data[point.y][point.x]
    }
}

#[derive(Debug, Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn relative(&self, x: isize, y: isize) -> Option<Self> {

        let x: usize = x.try_into().unwrap();
        let y: usize = y.try_into().unwrap();

        Point {
            x: self.x + x,
            y: self.y + y,
        }
    }
}

fn run_solution(solution: impl Solution, maze: &Maze, start: &Point, end: &Point) {
    let start_time = std::time::Instant::now();
    let result = solution.solve(maze, start, end);
    let duration = start_time.elapsed();
    println!(
        "Solution: {:12} took: {:8}ns, result: {:10}",
        solution.name(),
        duration.as_nanos(),
        result
    );
}

trait Solution {
    fn name(&self) -> String;
    fn solve(&self, maze: &Maze, start: &Point, end: &Point) -> bool;
}

struct BFSSolution;

impl Solution for BFSSolution {
    fn name(&self) -> String {
        String::from("BFSSolution")
    }

    fn solve(&self, maze: &Maze, start: &Point, end: &Point) -> bool {
        let mut queue = VecDeque::new();
        queue.push_front(start);

        while let Some(point) = queue.pop_front() {
            match maze.get_value(point) {
                Cell::Wall => {}
                Cell::Space => {
                    for offset in vec![(0, 1), (0, -1), (1, 0), (-1, 0)] {
                        let target = point.relative(offset.0, offset.1);
                    }
                }
            }
        }

        return true;
    }
}
