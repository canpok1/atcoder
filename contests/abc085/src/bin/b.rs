fn main() {
    let n: usize = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        line.trim_end().parse().unwrap()
    };

    let mut ans = 0;
    let mut mochi_list: Vec<usize> = Vec::new();
    for _ in 0..n {
        let mochi: usize = {
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            line.trim_end().parse().unwrap()
        };
        if mochi_list.contains(&mochi) {
            continue;
        }

        mochi_list.push(mochi);
        ans = ans + 1;
    }

    println!("{}", ans)
}
