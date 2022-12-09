use std::collections::HashMap;

use aoc::make_main;

const ENDING: [u32; 10] = [
    0b10000_10000_10000_10000_10000,
    0b01000_01000_01000_01000_01000,
    0b00100_00100_00100_00100_00100,
    0b00010_00010_00010_00010_00010,
    0b00001_00001_00001_00001_00001,
    0b11111_00000_00000_00000_00000,
    0b00000_11111_00000_00000_00000,
    0b00000_00000_11111_00000_00000,
    0b00000_00000_00000_11111_00000,
    0b00000_00000_00000_00000_11111,
    // 0b10000_01000_00100_00010_00001,
    // 0b00001_00010_00100_01000_10000,
];

struct Board {
    pos: HashMap<usize, usize>,
    marking: u32,
}
impl Board {
    fn mark(&mut self, x: usize) -> bool {
        self.pos.remove_entry(&x).and_then(|(_, y)| {
            self.marking |= 1 << y;
            Some(())
        });
        for &ending in ENDING.iter() {
            if self.marking & ending == ending {
                return true;
            }
        }
        false
    }
}

fn compute(input: String) -> usize {
    let (order, boards) = input.split_once("\n\n").unwrap();
    let order: Vec<usize> = order.split(",").map(|x| x.parse().unwrap()).collect();
    let mut boards: Vec<Board> = boards
        .split("\n\n")
        .map(|x| {
            let mut pos = HashMap::new();
            for (i, c) in x.split_whitespace().enumerate() {
                pos.insert(c.parse().unwrap(), i);
            }
            Board { marking: 0, pos }
        })
        .collect();
    let mut won = vec![false; boards.len()];
    let mut ongoing = boards.len();
    for i in order {
        for (j, board) in boards.iter_mut().enumerate() {
            if board.mark(i) && !won[j] {
                won[j] = true;
                ongoing -= 1;
                if ongoing == 0 {
                    return board.pos.keys().sum::<usize>() * i;
                }
            }
        }
    }
    panic!("No solution found");
}

make_main! { 1924 }
