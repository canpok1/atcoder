#[allow(dead_code)]
fn read_line() -> String {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.trim_end().to_owned()
}

fn main() {
    let s = read_line();

    let mut n: i64 = s.parse().unwrap();
    let mut sum = 0;
    let mut count_1 = 0;
    let mut count_2 = 0;
    while n > 0 {
        let k = n % 10;
        n = n / 10;

        match k % 3 {
            1 => {
                count_1 += 1;
            }
            2 => {
                count_2 += 1;
            }
            _ => {}
        }

        sum += k;
    }

    let ans = match sum % 3 {
        1 => {
            if count_1 > 0 && s.len() > 1 {
                1
            } else if count_1 == 0 && s.len() > 2 {
                2
            } else {
                -1
            }
        }
        2 => {
            if count_2 > 0 && s.len() > 1 {
                1
            } else if count_2 == 0 && s.len() > 2 {
                2
            } else {
                -1
            }
        }
        _ => 0,
    };
    println!("{}", ans);
}
