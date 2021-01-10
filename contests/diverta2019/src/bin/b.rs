fn main() {
    let (r, g, b, n) = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        let mut ws = line.trim_end().split_whitespace();
        let n1: isize = ws.next().unwrap().parse().unwrap();
        let n2: isize = ws.next().unwrap().parse().unwrap();
        let n3: isize = ws.next().unwrap().parse().unwrap();
        let n4: isize = ws.next().unwrap().parse().unwrap();
        (n1, n2, n3, n4)
    };
    let stdout = solve(r, g, b, n);
    stdout.iter().for_each(|s| {
        println!("{}", s);
    })
}

fn solve(r: isize, g: isize, b: isize, n: isize) -> Vec<String> {
    let mut ans = 0;

    let mut count_r = n / r;
    while count_r >= 0 {
        let mut count_g = (n - r * count_r) / g;
        while count_g >= 0 {
            let count_b = (n - r * count_r - g * count_g) / b;
            if r * count_r + g * count_g + b * count_b == n {
                ans += 1;
            }
            count_g -= 1;
        }
        count_r -= 1;
    }

    let mut buf = Vec::new();
    buf.push(format!("{}", ans));
    buf
}

#[test]
fn test_solve_1() {
    assert_eq!(solve(1, 2, 3, 4), vec!("4"));
}

#[test]
fn test_solve_2() {
    assert_eq!(solve(13, 1, 4, 3000), vec!("87058"));
}
