fn main() {
    let (n, x) = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        let mut ws = line.trim_end().split_whitespace();
        let n1: isize = ws.next().unwrap().parse().unwrap();
        let n2: isize = ws.next().unwrap().parse().unwrap();
        (n1, n2)
    };

    let s: Vec<char> = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        let s = line.trim_end().to_owned();
        s.chars().collect()
    };

    let mut p = x;
    (0_usize..(n as usize)).for_each(|index| {
        if s[index] == 'o' {
            p += 1;
        } else if p > 0 {
            p -= 1;
        }
    });

    println!("{}", p);
}
