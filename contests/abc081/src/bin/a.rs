fn main() {
    let s = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        line.trim_end().to_owned()
    };

    let mut count = 0;
    for c in s.chars() {
        if c == '1' {
            count = count + 1;
        }
    }
    println!("{}", count);
}
