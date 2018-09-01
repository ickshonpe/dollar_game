struct Board {    
    values: Vec<i64>,
    neighbours: Vec<Vec<usize>>
}

impl Board {
    pub fn send(&mut self, sender: usize, amount: i64) {
       self.values[sender] -= amount * self.neighbours[sender].len() as i64;
       for &a in &self.neighbours[sender] {
           self.values[a] += amount;
       }
    }

    pub fn print(&self) {
        for (i, (v, ns)) in self.values.iter().zip(self.neighbours.iter()).enumerate() {
            println!("node: {}, value: {}, neighbours: {:?}", i, v, ns);
        }
    }

    pub fn solved(&self) -> bool {
        self.values.iter().all(|&v| 0 <= v)
    }
}

fn main() {
    let mut board = Board {
        values: vec![1, -1],
        neighbours: vec![vec![1], vec![0]]
    };   
    board.print();
    while !board.solved() {        
        let (index, value): (usize, i64) = {
            let mut buf = String::new();
            let _ = std::io::stdin().read_line(&mut buf);
            let mut inputs = buf.split_whitespace();
            (inputs.next().unwrap().parse().unwrap(), inputs.next().unwrap().parse().unwrap())
        };
        board.send(index, value);
        board.print();
    }
    println!("You win!");    
}
