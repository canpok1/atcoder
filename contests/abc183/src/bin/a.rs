fn main() {
    let x: isize = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        line.trim_end().parse().unwrap()
    };
    if x >= 0 {
        println!("{}", x)
    } else {
        println!("{}", 0)
    }
}
