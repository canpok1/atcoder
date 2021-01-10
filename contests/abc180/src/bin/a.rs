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

    let (n, a, b) = {
        let mut ws = stdin[0].trim_end().split_whitespace();
        let n: usize = ws.next().unwrap().parse().unwrap();
        let a: usize = ws.next().unwrap().parse().unwrap();
        let b: usize = ws.next().unwrap().parse().unwrap();
        (n, a, b)
    };
    buf.push(format!("{}", n - a + b));

    buf
}

#[test]
fn test_run_sample1() {
    let input: Vec<String> = vec!["100 1 2"]
        .iter()
        .map(|v| v.to_string())
        .collect();
    assert_eq!(run(input), vec!("101"));
}

#[test]
fn test_run_sample2() {
    let input: Vec<String> = vec!["100 2 1"]
        .iter()
        .map(|v| v.to_string())
        .collect();
    assert_eq!(run(input), vec!("99"));
}

#[test]
fn test_run_sample3() {
    let input: Vec<String> = vec!["100 1 1"]
        .iter()
        .map(|v| v.to_string())
        .collect();
    assert_eq!(run(input), vec!("100"));
}
