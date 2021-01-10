fn main() {
    let _n: i32 = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        line.trim_end().parse().unwrap()
    };

    let mut a: Vec<usize> = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        line.split_whitespace()
            .map(|v| v.parse().unwrap())
            .collect()
    };
    a.sort_by(|a, b| b.cmp(a));

    let mut p_alice: usize = 0;
    let mut p_bob: usize = 0;
    let mut is_alice_turn = true;

    while a.len() > 0 {
        let p = a.remove(0);
        if is_alice_turn {
            p_alice = p_alice + p;
        } else {
            p_bob = p_bob + p;
        }
        is_alice_turn = !is_alice_turn;
    }

    println!("{}", p_alice - p_bob);
}
