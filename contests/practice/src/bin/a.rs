fn main() {
    let (a, b, c, s) = {
        let a: i32 = {
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            line.trim_end().parse().unwrap()
        };

        let (b, c) = {
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            let mut ws = line.trim_end().split_whitespace();
            let b: i32 = ws.next().unwrap().parse().unwrap();
            let c: i32 = ws.next().unwrap().parse().unwrap();
            (b, c)
        };

        let s = {
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            line.trim_end().to_owned()
        };

        (a, b, c, s)
    };

    println!("{} {}", a + b + c, s);
}
