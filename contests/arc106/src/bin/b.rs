use std::collections::HashSet;
use std::collections::HashMap;

fn main() {
    let mut stdin: Vec<String> = Vec::new();
    loop {
        let mut s = String::new();
        let size = std::io::stdin().read_line(&mut s).unwrap();
        if size == 0 {
            break;
        }
        stdin.push(s.trim_end().to_string());
    }

    run(stdin).iter().for_each(|v| println!("{}", v));
}

fn run(stdin: Vec<String>) -> Vec<String> {
    let mut buf = Vec::new();
    let (n, _m) = {
        let mut ws = stdin[0].trim_end().split_whitespace();
        let n1: usize = ws.next().unwrap().parse().unwrap();
        let n2: usize = ws.next().unwrap().parse().unwrap();
        (n1, n2)
    };

    let mut nodes1: Vec<Node> = Vec::new();
    for (i, a) in stdin[1].trim_end().split_whitespace().enumerate() {
        nodes1.push(Node{no: i + 1, cost: a.parse().unwrap()});
    };

    let mut nodes2: Vec<Node> = Vec::new();
    for (i, a) in stdin[2].trim_end().split_whitespace().enumerate() {
        nodes2.push(Node{no: i + 1, cost: a.parse().unwrap()});
    };

    // 隣接リスト (k, v) = (ノード番号, 隣接するノード番号のリスト)
    let mut ad_list: HashMap<usize, Vec<usize>> = HashMap::new();
    (&stdin[3..]).iter().for_each(|line| {
        let splited: Vec<usize> = line.trim_end().split_whitespace().map(|v| v.parse().unwrap()).collect();
        let (c, d) = (splited[0], splited[1]);
        match ad_list.get_mut(&c) {
            Some(v) => v.push(d),
            None => {ad_list.insert(c, vec![d]);},
        };
        match ad_list.get_mut(&d) {
            Some(v) => v.push(c),
            None => {ad_list.insert(d, vec![c]);},
        };
    });

    let solver = Solver {
        n: n,
        nodes1: &nodes1,
        nodes2: &nodes2,
        ad_list: &ad_list
    };

    if solver.solve() {
        buf.push(format!("Yes"));
    } else {
        buf.push(format!("No"));
    }
    buf
}

#[derive(Debug)]
struct Node {
    no: usize,
    cost: isize,
}

struct Solver<'a> {
    n: usize,
    nodes1: &'a Vec<Node>,
    nodes2: &'a Vec<Node>,
    ad_list: &'a HashMap<usize, Vec<usize>>,
}

impl Solver<'_> {
    fn solve(&self) -> bool {
        let mut calculated_set: HashSet<usize> = HashSet::new();

        let mut answer: bool = true;
        for no in 1..=self.n {
            if calculated_set.contains(&no) {
                continue;
            }

            let mut skip_set: HashSet<usize> = HashSet::new();
            let (sum1, sum2) = self.calc_cost_recursive(&no, &mut skip_set);
            if sum1 != sum2 {
                answer = false;
                break;
            }
            skip_set.iter().for_each(|no| {calculated_set.insert(*no);});
        }

        answer
    }

    fn calc_cost_recursive(&self, node_no: &usize, skip_set: &mut HashSet<usize>) -> (isize, isize) {
        if skip_set.contains(&node_no) {
            return (0, 0)
        }

        skip_set.insert(*node_no);
        let node_index = *node_no - 1;
        let mut sum1: isize = match self.nodes1.get(node_index) {
            Some(node) => node.cost,
            None => 0,
        };
        let mut sum2: isize = match self.nodes2.get(node_index) {
            Some(node) => node.cost,
            None => 0,
        };

        match self.ad_list.get(&node_no) {
            Some(others) => {
                others.iter().for_each(|other| {
                    let (cost1, cost2) = self.calc_cost_recursive(other, skip_set);
                    sum1 += cost1;
                    sum2 += cost2;
                });
            },
            None => ()
        };

        (sum1, sum2)
    }
}


#[test]
fn test_run_sample_1() {
    let input: Vec<String> = vec!["3 2", "1 2 3", "2 2 2", "1 2", "2 3"]
        .iter()
        .map(|v| v.to_string())
        .collect();
    assert_eq!(run(input), vec!("Yes"));
}

#[test]
fn test_run_sample_2() {
    let input: Vec<String> = vec!["1 0", "5", "5"]
        .iter()
        .map(|v| v.to_string())
        .collect();
    assert_eq!(run(input), vec!("Yes"));
}

#[test]
fn test_run_sample_3() {
    let input: Vec<String> = vec!["2 1", "1 1", "2 1", "1 2"]
        .iter()
        .map(|v| v.to_string())
        .collect();
    assert_eq!(run(input), vec!("No"));
}

#[test]
fn test_run_sample_4() {
    let input: Vec<String> = vec![
        "17 9",
        "-905371741 -999219903 969314057 -989982132 -87720225 -175700172 -993990465 929461728 895449935 -999016241 782467448 -906404298 578539175 9684413 -619191091 -952046546 125053320",
        "-440503430 -997661446 -912471383 -995879434 932992245 -928388880 -616761933 929461728 210953513 -994677396 648190629 -530944122 578539175 9684413 595786809 -952046546 125053320",
        "2 10",
        "6 12",
        "9 11",
        "11 5",
        "7 6",
        "3 15",
        "3 1",
        "1 9",
        "10 4",
    ].iter().map(|v| v.to_string()).collect();
    assert_eq!(run(input), vec!("Yes"));
}
