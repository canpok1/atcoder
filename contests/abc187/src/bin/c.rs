use std::collections::HashMap;

fn main() {
    let n: usize = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        line.trim_end().parse().unwrap()
    };

    let mut ss: Vec<String> = Vec::new();
    (0..n).for_each(|_| {
        let s: String = {
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            line.trim_end().to_owned()
        };
        ss.push(s);
    });

    let stdout = solve(ss);
    stdout.iter().for_each(|s| {
        println!("{}", s);
    })
}

struct Exist {
    keyword: bool,
    keyword_not: bool,
}

fn solve(ss: Vec<String>) -> Vec<String> {
    let mut exists: HashMap<String, Exist> = HashMap::new();

    for s in ss {
        let keyword = s.replace("!", "");
        match exists.get_mut(&keyword) {
            Some(exist) => {
                if s.chars().nth(0).unwrap() == '!' {
                    exist.keyword_not = true;
                } else {
                    exist.keyword = true;
                }
            }
            None => {
                let mut exist = Exist {
                    keyword: false,
                    keyword_not: false,
                };
                if s.chars().nth(0).unwrap() == '!' {
                    exist.keyword_not = true;
                } else {
                    exist.keyword = true;
                }
                exists.insert(keyword, exist);
            }
        }
    }

    let mut ans: String = "satisfiable".to_string();
    for (k, exist) in exists {
        if exist.keyword && exist.keyword_not {
            ans = k;
        }
    }

    let mut buf = Vec::new();
    buf.push(format!("{}", ans));
    buf
}

#[test]
fn test_solve_1() {
    let ss = vec![
        "a".to_string(),
        "!a".to_string(),
        "b".to_string(),
        "!c".to_string(),
        "d".to_string(),
        "!d".to_string(),
    ];
    assert_eq!(solve(ss), vec!("a"));
}

#[test]
fn test_solve_2() {
    let ss = vec![
        "red".to_string(),
        "red".to_string(),
        "red".to_string(),
        "!orange".to_string(),
        "yellow".to_string(),
        "!blue".to_string(),
        "cyan".to_string(),
        "!green".to_string(),
        "brown".to_string(),
        "!gray".to_string(),
    ];

    assert_eq!(solve(ss), vec!("satisfiable"));
}
