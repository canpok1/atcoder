use std::collections::BTreeSet;

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

    let n: u64 = {
        let mut ws = stdin[0].trim_end().split_whitespace();
        ws.next().unwrap().parse().unwrap()
    };

    let mut ans: BTreeSet<u64> = BTreeSet::new();
    let max = n / 2;
    for v in 1..=max {
        if n % v == 0 {
            ans.insert(v);
            ans.insert(n / v);
        }
    }

    ans.iter().for_each(|v| buf.push(format!("{}", v)));
    buf
}

#[test]
fn test_run_sample1() {
    let input: Vec<String> = vec!["6"].iter().map(|v| v.to_string()).collect();
    assert_eq!(run(input), vec!("1", "2", "3", "6"));
}

#[test]
fn test_run_sample2() {
    let input: Vec<String> = vec!["720"].iter().map(|v| v.to_string()).collect();
    assert_eq!(
        run(input),
        vec!(
            "1", "2", "3", "4", "5", "6", "8", "9", "10", "12", "15", "16", "18", "20", "24", "30",
            "36", "40", "45", "48", "60", "72", "80", "90", "120", "144", "180", "240", "360",
            "720"
        )
    );
}

#[test]
fn test_run_sample3() {
    let input: Vec<String> = vec!["1000000007"].iter().map(|v| v.to_string()).collect();
    assert_eq!(run(input), vec!("1", "1000000007"));
}
