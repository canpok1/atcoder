fn main() {
    let _n: u32 = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        let mut ws = line.trim_end().split_whitespace();
        ws.next().unwrap().parse().unwrap()
    };

    let mut board: Board = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        Board::new(&line)
    };

    while board.can_update() {
        board.update();
    }

    println!("{}", board.get_count());
}

#[derive(Debug)]
struct Board {
    a: Vec<u32>,
    update_count: u32,
}

impl Board {
    fn new(line: &str) -> Board {
        let a = line.trim_end().split_whitespace().map(|v| v.parse().unwrap()).collect();
        Board { a: a, update_count: 0 }
    }

    fn can_update(&self) -> bool {
        for val in self.a.iter() {
            if (val % 2) == 1 {
                return false;
            }
        }
        true
    }

    fn update(&mut self) {
        self.a = self.a.iter().map(|v| v / 2).collect();
        self.update_count = self.update_count + 1;
    }

    fn get_count(&self) -> u32 {
        self.update_count
    }
}