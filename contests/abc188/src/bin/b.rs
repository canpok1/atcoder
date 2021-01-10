fn main() {
    let n = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        line.trim_end().parse().unwrap()
    };
    let a = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        line.trim_end()
            .split_whitespace()
            .map(|v| v.parse().unwrap())
            .collect()
    };
    let b = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        line.trim_end()
            .split_whitespace()
            .map(|v| v.parse().unwrap())
            .collect()
    };

    let stdout = solve(n, a, b);
    stdout.iter().for_each(|s| {
        println!("{}", s);
    })
}

fn solve(n: usize, a: Vec<isize>, b: Vec<isize>) -> Vec<String> {
    let mut ans = 0;
    (0..n).for_each(|i| {
        ans += (a[i] as i64) * (b[i] as i64);
    });

    let mut buf = Vec::new();
    if ans == 0 {
        buf.push(format!("Yes"));
    } else {
        buf.push(format!("No"));
    }
    buf
}
