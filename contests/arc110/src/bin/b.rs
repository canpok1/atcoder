fn main() {
    let n: usize = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        line.trim_end().parse().unwrap()
    };
    let t: String = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        line.trim_end().to_owned()
    };

    for o in run(n, t) {
        println!("{}", o);
    }
}

fn run(n: usize, t: String) -> Vec<String> {
    let mut buf = Vec::new();
    if t == "1" {
        buf.push(format!("{}", 10_u64.pow(10) * 2));
        return buf;
    }
    if t == "11" {
        buf.push(format!("{}", 10_u64.pow(10)));
        return buf;
    }

    let mut s: String = "110".to_string();
    loop {
        if s.len() >= n + 2 {
            break;
        }
        s = format!("{}110", s);
    }

    let mut zero_count = 0;
    t.chars().for_each(|c| {
        if c == '0' {
            zero_count += 1;
        }
    });

    let ans = if s[0..n] == t || s[1..n + 1] == t || s[2..n + 2] == t {
        if t.chars().last().unwrap() == '0' {
            10_u64.pow(10) - zero_count + 1
        } else {
            10_u64.pow(10) - zero_count
        }
    } else {
        0
    };

    buf.push(format!("{}", ans));
    buf
}

#[test]
fn test_run_1() {
    assert_eq!(run(4, "1011".to_string()), vec!("9999999999"));
}

#[test]
fn test_run_2() {
    assert_eq!(
        run(22, "1011011011011011011011".to_string()),
        vec!("9999999993")
    );
}

#[test]
fn test_run_3() {
    assert_eq!(run(1, "1".to_string()), vec!("20000000000"));
}

#[test]
fn test_run_4() {
    assert_eq!(run(1, "0".to_string()), vec!("10000000000"));
}

#[test]
fn test_run_5() {
    assert_eq!(run(2, "00".to_string()), vec!("0"));
}

#[test]
fn test_run_6() {
    assert_eq!(run(2, "01".to_string()), vec!("9999999999"));
}

#[test]
fn test_run_7() {
    assert_eq!(run(2, "10".to_string()), vec!("10000000000"));
}

#[test]
fn test_run_8() {
    assert_eq!(run(2, "11".to_string()), vec!("10000000000"));
}

#[test]
fn test_run_9() {
    assert_eq!(run(3, "000".to_string()), vec!("0"));
}

#[test]
fn test_run_10() {
    assert_eq!(run(3, "001".to_string()), vec!("0"));
}

#[test]
fn test_run_11() {
    assert_eq!(run(3, "010".to_string()), vec!("0"));
}

#[test]
fn test_run_12() {
    assert_eq!(run(3, "011".to_string()), vec!("9999999999"));
}

#[test]
fn test_run_13() {
    assert_eq!(run(3, "100".to_string()), vec!("0"));
}

#[test]
fn test_run_14() {
    assert_eq!(run(3, "101".to_string()), vec!("9999999999"));
}

#[test]
fn test_run_15() {
    assert_eq!(run(3, "110".to_string()), vec!("10000000000"));
}

#[test]
fn test_run_16() {
    assert_eq!(run(3, "111".to_string()), vec!("0"));
}

#[test]
fn test_run_17() {
    assert_eq!(run(4, "0110".to_string()), vec!("9999999999"));
}

#[test]
fn test_run_18() {
    assert_eq!(run(9, "110110110".to_string()), vec!("9999999998"));
}
