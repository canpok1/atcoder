#[allow(dead_code)]
fn read_line() -> String {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.trim_end().to_owned()
}

struct Piece {
    x: i64,
    y: i64,
}

struct Op {
    code: usize,
    p: i64,
}

impl Op {
    fn new(param: Vec<isize>) -> Op {
        Op {
            code: param[0] as usize,
            p: if param.len() == 2 { param[1] as i64 } else { 0 },
        }
    }
}

type Matrix3 = Vec<Vec<i64>>;
fn matmul(m1: &Matrix3, m2: &Matrix3) -> Matrix3 {
    vec![
        vec![
            m1[0][0] * m2[0][0] + m1[0][1] * m2[1][0] + m1[0][2] * m2[2][0],
            m1[0][0] * m2[0][1] + m1[0][1] * m2[1][1] + m1[0][2] * m2[2][1],
            m1[0][0] * m2[0][2] + m1[0][1] * m2[1][2] + m1[0][2] * m2[2][2],
        ],
        vec![
            m1[1][0] * m2[0][0] + m1[1][1] * m2[1][0] + m1[1][2] * m2[2][0],
            m1[1][0] * m2[0][1] + m1[1][1] * m2[1][1] + m1[1][2] * m2[2][1],
            m1[1][0] * m2[0][2] + m1[1][1] * m2[1][2] + m1[1][2] * m2[2][2],
        ],
        vec![
            m1[2][0] * m2[0][0] + m1[2][1] * m2[1][0] + m1[2][2] * m2[2][0],
            m1[2][0] * m2[0][1] + m1[2][1] * m2[1][1] + m1[2][2] * m2[2][1],
            m1[2][0] * m2[0][2] + m1[2][1] * m2[1][2] + m1[2][2] * m2[2][2],
        ],
    ]
}

fn unit_matrix3() -> Matrix3 {
    vec![vec![1, 0, 0], vec![0, 1, 0], vec![0, 0, 1]]
}

fn main() {
    let n = read_line().parse().unwrap();
    let mut pieces: Vec<Piece> = Vec::new();
    for _ in 0..n {
        let stdin = read_line();
        let mut iter = stdin.split_whitespace();
        let x = iter.next().unwrap().parse().unwrap();
        let y = iter.next().unwrap().parse().unwrap();
        pieces.push(Piece { x: x, y: y });
    }
    let m = read_line().parse().unwrap();
    let mut ops: Vec<Op> = Vec::new();
    for _ in 0..m {
        let op = read_line()
            .split_whitespace()
            .map(|v| v.parse().unwrap())
            .collect();
        ops.push(Op::new(op));
    }

    let solver = Solver::new(pieces, ops);

    let q = read_line().parse().unwrap();
    for _ in 0..q {
        let stdin = read_line();
        let mut iter = stdin.split_whitespace();
        let a = iter.next().unwrap().parse().unwrap();
        let b = iter.next().unwrap().parse().unwrap();

        let (x, y) = solver.query(a, b);

        println!("{} {}", x, y);
    }
}

struct Solver {
    pieces: Vec<Piece>,
    matrixes: Vec<Matrix3>,
}
impl Solver {
    fn new(pieces: Vec<Piece>, ops: Vec<Op>) -> Solver {
        let mut muls: Vec<Matrix3> = vec![unit_matrix3()];
        for op in ops {
            let m1 = match op.code {
                1 => vec![vec![0, 1, 0], vec![-1, 0, 0], vec![0, 0, 1]],
                2 => vec![vec![0, -1, 0], vec![1, 0, 0], vec![0, 0, 1]],
                3 => vec![vec![-1, 0, 2 * op.p], vec![0, 1, 0], vec![0, 0, 1]],
                _ => vec![vec![1, 0, 0], vec![0, -1, 2 * op.p], vec![0, 0, 1]],
            };
            let m2 = muls.last().unwrap();
            let mul = matmul(&m1, m2);
            muls.push(mul);
        }

        Solver {
            pieces: pieces,
            matrixes: muls,
        }
    }

    fn query(&self, a: usize, b: usize) -> (i64, i64) {
        let p = &self.pieces[b - 1];
        let m = &self.matrixes[a];
        (
            m[0][0] * p.x + m[0][1] * p.y + m[0][2],
            m[1][0] * p.x + m[1][1] * p.y + m[1][2],
        )
    }
}
