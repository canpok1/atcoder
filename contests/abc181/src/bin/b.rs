fn main() {
    let n: usize = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        line.trim_end().parse().unwrap()
    };
    let mut sum: u64 = 0;
    (0..n).for_each(|_| {
        let (a, b) = {
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            let mut ws = line.trim_end().split_whitespace();
            let n1: u64 = ws.next().unwrap().parse().unwrap();
            let n2: u64 = ws.next().unwrap().parse().unwrap();
            (n1, n2)
        };
        (a..=b).for_each(|v| sum += v);
    });
    println!("{}", sum);
}
