fn main() {
    let (sx, sy, gx, gy) = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        let mut ws = line.trim_end().split_whitespace();
        let sx: f64 = ws.next().unwrap().parse().unwrap();
        let sy: f64 = ws.next().unwrap().parse().unwrap();
        let gx: f64 = ws.next().unwrap().parse().unwrap();
        let gy: f64 = ws.next().unwrap().parse().unwrap();
        (sx, sy, gx, gy)
    };

    let a: f64 = (-gy - sy) / (gx - sx);
    let b: f64 = sy - a * sx;
    let ans: f64 = -b / a;

    println!("{:.10}", ans);
}
