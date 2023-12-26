use aoc::make_main;

fn compute(input: String) -> usize {
    let mut board = vec![vec![0; 1000]; 1000];
    for line in input.lines() {
        let (p1, p2) = line.split_once(" -> ").unwrap();
        let (x1, y1) = p1.split_once(',').unwrap();
        let (x2, y2) = p2.split_once(',').unwrap();
        let mut x1 = x1.parse::<usize>().unwrap();
        let mut y1 = y1.parse::<usize>().unwrap();
        let mut x2 = x2.parse::<usize>().unwrap();
        let mut y2 = y2.parse::<usize>().unwrap();
        if x1 == x2 || y1 == y2 {
            if x1 > x2 {
                std::mem::swap(&mut x1, &mut x2);
            }
            if y1 > y2 {
                std::mem::swap(&mut y1, &mut y2);
            }
            for x in x1..=x2 {
                for y in y1..=y2 {
                    board[x][y] += 1;
                }
            }
        }
    }
    board.into_iter().flatten().filter(|&x| x > 1).count()
}

make_main! { 5 }
