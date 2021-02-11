use std::cmp::min;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;

#[allow(dead_code)]
fn read_line() -> String {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.trim_end().to_owned()
}

fn main() {
    let stdin = read_line();
    let mut iter = stdin.split_whitespace();
    let n = iter.next().unwrap().parse().unwrap();
    let m = iter.next().unwrap().parse().unwrap();

    let mut roads: Vec<Road> = Vec::new();
    for _ in 0..m {
        let stdin = read_line();
        let mut iter = stdin.split_whitespace();
        let a: usize = iter.next().unwrap().parse().unwrap();
        let b: usize = iter.next().unwrap().parse().unwrap();
        let c: i64 = iter.next().unwrap().parse().unwrap();
        roads.push(Road {
            a: a - 1,
            b: b - 1,
            c: c,
        });
    }

    let solver = Solver::new(n, roads);
    let stdout = solver.solve();
    stdout.iter().for_each(|s| {
        println!("{}", s);
    });
}

#[derive(PartialEq, Eq)]
struct Target {
    cost: i64,
    index: usize,
}

impl Ord for Target {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Target {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct Road {
    a: usize,
    b: usize,
    c: i64,
}

struct Solver {
    n: usize,
    roads: Vec<Road>,
    road_idx_by_a: HashMap<usize, Vec<usize>>,
    road_idx_by_b: HashMap<usize, Vec<usize>>,
}

impl Solver {
    fn new(n: usize, roads: Vec<Road>) -> Solver {
        let mut ra: HashMap<usize, Vec<usize>> = HashMap::new();
        let mut rb: HashMap<usize, Vec<usize>> = HashMap::new();
        for (i, r) in roads.iter().enumerate() {
            if ra.contains_key(&r.a) {
                let v = ra.get_mut(&r.a).unwrap();
                v.push(i);
            } else {
                ra.insert(r.a, vec![i]);
            }
            if rb.contains_key(&r.b) {
                let v = rb.get_mut(&r.b).unwrap();
                v.push(i);
            } else {
                rb.insert(r.b, vec![i]);
            }
        }

        Solver {
            n: n,
            roads: roads,
            road_idx_by_a: ra,
            road_idx_by_b: rb,
        }
    }

    fn solve_at(&self, start: usize) -> i64 {
        let mut costs: Vec<_> = (0..self.n).map(|_| std::i64::MAX).collect();
        costs[start] = 0;

        let mut queue = BinaryHeap::new();
        queue.push(Target {
            cost: 0,
            index: start,
        });
        while let Some(target) = queue.pop() {
            if target.cost > costs[target.index] {
                continue;
            }

            if let Some(road_idxs) = self.road_idx_by_a.get(&target.index) {
                for i in road_idxs {
                    let road = &self.roads[*i];
                    let next = Target {
                        cost: target.cost + road.c,
                        index: road.b,
                    };
                    if next.cost < costs[next.index] {
                        costs[next.index] = next.cost;
                        queue.push(next);
                    }
                }
            }
        }

        let mut ans = std::i64::MAX;
        if let Some(road_idxs) = self.road_idx_by_b.get(&start) {
            for i in road_idxs {
                let road = &self.roads[*i];
                if costs[road.a] == std::i64::MAX {
                    continue;
                }

                let c = costs[road.a] + road.c;
                ans = min(ans, c);
            }
        }

        if ans == std::i64::MAX {
            -1
        } else {
            ans
        }
    }

    fn solve(&self) -> Vec<String> {
        let mut buf = Vec::new();
        for start in 0..self.n {
            let ans = self.solve_at(start);
            buf.push(format!("{}", ans));
        }
        buf
    }
}
