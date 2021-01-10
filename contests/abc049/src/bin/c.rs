fn main() {
    let s: String = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        line.trim_end().to_owned()
    };

    let mut t = String::new();
    let append_txts = ["dream", "dreamer", "erase", "eraser"];

    while t.len() < s.len() {
        for append_txt in &append_txts {
            let t_copy = format!("{}{}", t, append_txt);
            if t_copy.len() > s.len() {
                println!("NO");
                return;
            }

            let s_copy = &s[0..t_copy.len()];
            if t_copy == s_copy {
                t = t_copy;
                break;
            }
        }
    }

    if t.len() == s.len() {
        println!("YES");
    } else {
        println!("NO");
    }
}
