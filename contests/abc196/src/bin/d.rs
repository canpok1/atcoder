use proconio::input;
fn main() {
    let mut solver = Solver::new();
    println!("{}", solver.solve());
}

struct Solver {
    h: isize,
    w: isize,
    a: isize,
    b: isize,
    ans: i64,
}

impl Solver {
    fn new() -> Solver {
        input! {
            h: isize,
            w: isize,
            a: isize,
            b: isize,
        }
        Solver {
            h: h,
            w: w,
            a: a,
            b: b,
            ans: 0,
        }
    }

    fn solve(&mut self) -> i64 {
        self.ans = 0;
        self.put(0, 0, self.a, self.b);
        self.ans
    }

    fn put(&mut self, tatamis: u32, idx: isize, a: isize, b: isize) {
        if idx == self.h * self.w {
            self.ans += 1;
            return;
        }

        if (tatamis >> idx) & 1 == 1 {
            self.put(tatamis, idx + 1, a, b);
            return;
        }

        if a > 0 {
            let tatami_a1 = 1 << idx;

            // 横（右端でないことを考慮）
            let tatami_a2_idx = idx + 1;
            if (idx % self.w != self.w - 1) && (tatamis >> tatami_a2_idx) & 1 == 0 {
                let tatami_a2 = 1 << tatami_a2_idx;
                self.put(tatamis | tatami_a1 | tatami_a2, idx + 1, a - 1, b);
            }

            // 縦（下端でないことを考慮）
            let tatami_a2_idx = idx + self.w;
            if (idx / self.w < self.h - 1) && (tatamis >> tatami_a2_idx) & 1 == 0 {
                let tatami_a2 = 1 << tatami_a2_idx;
                self.put(tatamis | tatami_a1 | tatami_a2, idx + 1, a - 1, b);
            }
        }

        if b > 0 {
            let tatami_b = 1 << idx;
            self.put(tatamis | tatami_b, idx + 1, a, b - 1);
        }
    }
}
