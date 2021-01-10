fn main() {
    let (x, y) = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        let mut ws = line.trim_end().split_whitespace();
        let n1: isize = ws.next().unwrap().parse().unwrap();
        let n2: isize = ws.next().unwrap().parse().unwrap();
        (n1, n2)
    };

    let diff = x - y;
    if diff.abs() < 3 {
        println!("Yes");
    } else {
        println!("No");
    }
}
