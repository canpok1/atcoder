fn main() {
    let (a, b) = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        let mut ws = line.trim_end().split_whitespace();
        let a: usize = ws.next().unwrap().parse().unwrap();
        let b: usize = ws.next().unwrap().parse().unwrap();
        (a, b)
    };

    if (a * b) % 2 == 0 {
        println!("Even");
    } else {
        println!("Odd");
    }
}
