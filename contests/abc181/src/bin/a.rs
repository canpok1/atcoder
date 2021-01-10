fn main() {
    let n: usize = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        line.trim_end().parse().unwrap()
    };
    if n % 2 == 0 {
        println!("White");
    } else {
        println!("Black");
    }
}
