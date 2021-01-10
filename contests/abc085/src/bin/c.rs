fn main() {
    let (n, y) = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        let mut ws = line.trim_end().split_whitespace();
        let n1: isize = ws.next().unwrap().parse().unwrap();
        let n2: isize = ws.next().unwrap().parse().unwrap();
        (n1, n2)
    };

    let mut is_found = false;

    for c10000 in (0..=n).rev() {
        if is_found {
            break;
        }

        for c5000 in (0..=(n - c10000)).rev() {
            if is_found {
                break;
            }

            for c1000 in (0..=(n - c10000 - c5000)).rev() {
                if is_found {
                    break;
                }

                if (c10000 + c5000 + c1000) != n {
                    continue;
                }

                let sum = c10000 * 10000 + c5000 * 5000 + c1000 * 1000;
                if sum == y {
                    println!("{} {} {}", c10000, c5000, c1000);
                    is_found = true;
                }
            }
        }
    }

    if !is_found {
        println!("-1 -1 -1");
    }
}
