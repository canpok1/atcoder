use std::collections::HashMap;

fn main() {
    let s: String = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        line.trim_end().to_owned()
    };

    println!("{}", run(&s));
}

fn run(s: &str) -> String {
    let mut char_map_s: HashMap<char, usize> = HashMap::new();
    for c in s.chars() {
        let count = match char_map_s.get(&c) {
            Some(n) => n + 1,
            None => 1,
        };
        char_map_s.insert(c, count);
    }

    let mut can_make = false;

    let len = s.len() as u32;
    let min = 10_i32.pow(len - 1) / 8 + 1;
    let max = (10_i32.pow(len) - 1) / 8;

    for num_str in (min..=max).map(|n| format!("{}", 8 * n)) {
        let mut contains_0 = false;
        let mut char_map_8: HashMap<char, usize> = HashMap::new();
        for c in num_str.chars() {
            if c == '0' {
                contains_0 = true;
                break;
            }
            let count = match char_map_8.get(&c) {
                Some(n) => n + 1,
                None => 1,
            };
            char_map_8.insert(c, count);
        }

        if contains_0 {
            continue;
        }

        if char_map_s.keys().len() != char_map_8.keys().len() {
            continue;
        }

        let mut all_match = true;
        for key in char_map_s.keys() {
            if !char_map_8.contains_key(&key) {
                all_match = false;
                break;
            }
            if char_map_s.get(key).unwrap() != char_map_8.get(key).unwrap() {
                all_match = false;
                break;
            }
        }

        if all_match {
            can_make = true;
            break;
        }
    }

    if can_make {
        "Yes".to_string()
    } else {
        "No".to_string()
    }
}

#[test]
fn test_sample_1() {
    assert_eq!(run("1234"), "Yes");
}

#[test]
fn test_sample_2() {
    assert_eq!(run("1333"), "No");
}

#[test]
fn test_sample_3() {
    assert_eq!(run("8"), "Yes");
}
