fn main() {
    let (n, a, b) = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        let mut ws = line.trim_end().split_whitespace();
        let n: u32 = ws.next().unwrap().parse().unwrap();
        let a: u32 = ws.next().unwrap().parse().unwrap();
        let b: u32 = ws.next().unwrap().parse().unwrap();
        (n, a, b)
    };

    let mut ans = 0;
    for v in 1..n+1 {
        let splited: Vec<u32> = v.to_string().chars().map(|c| c.to_digit(10).unwrap()).collect();
        let mut sum = 0;
        for n in splited {
            sum = sum + n;
        }
        if sum >= a && sum <=b {
            ans = ans + v;
        }
    }

    println!("{}", ans);
}
