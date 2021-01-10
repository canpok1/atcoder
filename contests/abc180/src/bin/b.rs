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

    let x_list: Vec<isize> = {
        stdin[1].trim_end().split_whitespace().map(|v| v.parse().unwrap()).collect()
    };

    let mut m: i64 = 0;
    let mut y: i64 = 0;
    let mut c: i64 = 0;

    for x in x_list {
        let abs = x.abs() as i64;
        m += abs;
        y += abs * abs;
        c = if c < abs {
            abs
        } else {
            c
        };
    }

    buf.push(format!("{}", m));
    buf.push(format!("{}", (y as f64).sqrt()));
    buf.push(format!("{}", c));

    buf
}

#[test]
fn test_run_sample1() {
    let input: Vec<String> = vec!["2", "2 -1"]
        .iter()
        .map(|v| v.to_string())
        .collect();
    assert_eq!(run(input), vec!("3", "2.236067977499790", "2"));
}

#[test]
fn test_run_sample2() {
    let input: Vec<String> = vec!["10", "3 -1 -4 1 -5 9 2 -6 5 -3"]
        .iter()
        .map(|v| v.to_string())
        .collect();
    assert_eq!(run(input), vec!("39", "14.387494569938159", "9"));
}
