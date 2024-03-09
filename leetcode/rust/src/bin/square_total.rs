fn main() {
    // print outs how long it takes to run the function

    let start = std::time::Instant::now();

    let result = square_total(100);

    println!("Time: {:?}", start.elapsed());

    for pair in result {
        pair.pretty_print();
    }
}

// list of pairs a^3 + b^3 = c^3 + d^3 where each is below than passed number n
fn square_total(n: u32) -> Vec<Pair> {
    let mut results: Vec<Pair> = Vec::new();

    // n >= a >= b >= 0 AND n >= c >= d >= 0

    for a in 0..n {
        for b in 0..a {
            for c in 0..n {
                for d in 0..c {
                    if a == c && b == d {
                        continue;
                    }
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

    results
}

#[derive(Debug)]
struct Pair {
    a: u32,
    b: u32,
    c: u32,
    d: u32,
    value: u32,
}

impl Pair {
    fn pretty_print(&self) {
        println!(
            "{:3}^3 + {:3}^3 = {:3}^3 + {:3}^3 = {}",
            self.a, self.b, self.c, self.d, self.value
        );
    }
}
