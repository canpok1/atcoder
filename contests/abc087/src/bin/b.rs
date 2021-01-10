fn main() {
    let (a, b, c, x) = {
        let a: usize = {
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            line.trim_end().parse().unwrap()
        };
        let b: usize = {
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            line.trim_end().parse().unwrap()
        };
        let c: usize = {
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            line.trim_end().parse().unwrap()
        };
        let x: usize = {
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            line.trim_end().parse().unwrap()
        };
        (a, b, c, x)
    };

    let mut ans = 0;

    for c500 in 0..(a+1) {
        for c100 in 0..(b+1) {
            for c50 in 0..(c+1) {
                if (c500*500 + c100*100 + c50*50) == x {
                    ans = ans + 1;
                }
            }
        }
    }
    println!("{}", ans);
}
