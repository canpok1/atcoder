fn main() {
    let (a, b) = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        let mut ws = line.trim_end().split_whitespace();
        let a: isize = ws.next().unwrap().parse().unwrap();
        let b: isize = ws.next().unwrap().parse().unwrap();
        (a, b)
    };
    let (c, d) = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        let mut ws = line.trim_end().split_whitespace();
        let c: isize = ws.next().unwrap().parse().unwrap();
        let d: isize = ws.next().unwrap().parse().unwrap();
        (c, d)
    };

    println!("{}", a * d - b * c);
}
