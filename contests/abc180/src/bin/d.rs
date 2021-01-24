fn main() {
    let (x, y, a, b) = {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap();
        let mut ws = s.trim_end().split_whitespace();
        let x = ws.next().unwrap().parse().unwrap();
        let y = ws.next().unwrap().parse().unwrap();
        let a = ws.next().unwrap().parse().unwrap();
        let b = ws.next().unwrap().parse().unwrap();
        (x, y, a, b)
    };

    println!("{}", run(x, y, a, b));
}

fn run(begin_x: i64, y: i64, a: i64, b: i64) -> String {
    let mut exp = 0;
    let mut x = begin_x;
    loop {
        let (next_x, overflow) = a.overflowing_mul(x);
        if overflow || next_x > x + b || next_x >= y {
            break;
        }
        x = next_x;
        exp += 1;
    }

    exp += (y - x - 1) / b;

    format!("{}", exp)
}

#[test]
fn test_run_sample1() {
    assert_eq!(run(4, 20, 2, 10), "2");
}

#[test]
fn test_run_sample2() {
    assert_eq!(run(1, 1000000000000000000, 10, 1000000000), "1000000007");
}

#[test]
fn test_run_sample3() {
    assert_eq!(run(4, 1, 10, 10), "0");
}

#[test]
fn test_run_sample4() {
    assert_eq!(run(4, 4, 10, 10), "0");
}

#[test]
fn test_run_sample5() {
    assert_eq!(run(4, 10, 2, 20), "1");
}

#[test]
fn test_run_sample6() {
    assert_eq!(run(4, 16, 2, 40), "1");
}

#[test]
fn test_run_sample7() {
    assert_eq!(run(6, 30, 2, 10), "2");
}
