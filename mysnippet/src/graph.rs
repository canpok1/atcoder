use cargo_snippet::snippet;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[snippet(name = "myfunc graph dijkstra")]
#[derive(Eq, PartialEq, PartialOrd)]
struct Node {
    cost: i64,
    index: usize,
}

#[snippet(name = "myfunc graph dijkstra")]
#[snippet(prefix = "use std::cmp::Ordering;")]
impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

#[snippet(name = "myfunc graph dijkstra")]
struct Edge {
    from: usize,
    to: usize,
    cost: i64,
}

#[snippet(name = "myfunc graph dijkstra")]
#[snippet(prefix = "use std::collections::BinaryHeap;")]
#[allow(dead_code)]
fn dijkstra(n: usize, edges: &Vec<Edge>, start: usize, goal: usize) -> Option<i64> {
    const INIT_COST: i64 = std::i64::MAX;
    let mut costs: Vec<_> = (0..n).map(|_| INIT_COST).collect();
    let mut queue = BinaryHeap::new();

    costs[start] = 0;
    queue.push(Node {
        cost: 0,
        index: start,
    });

    while let Some(current) = queue.pop() {
        if current.cost > costs[current.index] {
            continue;
        }
        for edge in edges {
            if edge.from != current.index {
                continue;
            }
            let next = Node {
                cost: current.cost + edge.cost,
                index: edge.to,
            };
            if next.cost < costs[next.index] {
                costs[next.index] = next.cost;
                queue.push(next);
            }
        }
    }

    if costs[goal] == INIT_COST {
        None
    } else {
        Some(costs[goal])
    }
}

#[test]
fn test_dijkstra() {
    let n = 5;
    let edges = vec![
        Edge {
            from: 0,
            to: 2,
            cost: 10,
        },
        Edge {
            from: 0,
            to: 1,
            cost: 1,
        },
        Edge {
            from: 1,
            to: 3,
            cost: 2,
        },
        Edge {
            from: 2,
            to: 1,
            cost: 1,
        },
        Edge {
            from: 2,
            to: 3,
            cost: 3,
        },
        Edge {
            from: 2,
            to: 4,
            cost: 1,
        },
        Edge {
            from: 3,
            to: 0,
            cost: 7,
        },
        Edge {
            from: 3,
            to: 4,
            cost: 2,
        },
    ];
    assert_eq!(dijkstra(n, &edges, 0, 1), Some(1));
    assert_eq!(dijkstra(n, &edges, 0, 3), Some(3));
    assert_eq!(dijkstra(n, &edges, 3, 0), Some(7));
    assert_eq!(dijkstra(n, &edges, 0, 4), Some(5));
    assert_eq!(dijkstra(n, &edges, 4, 0), None);
}
