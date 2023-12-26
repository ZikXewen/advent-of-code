use aoc::make_main;

fn sign(x: usize, y: usize) -> isize {
    match x.cmp(&y) {
        std::cmp::Ordering::Less => 1,
        std::cmp::Ordering::Equal => 0,
        std::cmp::Ordering::Greater => -1,
    }
}

fn compute(input: String) -> usize {
    let mut board = vec![vec![0; 1000]; 1000];
    for line in input.lines() {
        let (p1, p2) = line.split_once(" -> ").unwrap();
        let (x1, y1) = p1.split_once(',').unwrap();
        let (x2, y2) = p2.split_once(',').unwrap();
        let x1 = x1.parse::<usize>().unwrap();
        let y1 = y1.parse::<usize>().unwrap();
        let x2 = x2.parse::<usize>().unwrap();
        let y2 = y2.parse::<usize>().unwrap();
        let dx = sign(x1, x2);
        let dy = sign(y1, y2);
        let mut x = x1;
        let mut y = y1;
        while x != x2 || y != y2 {
            board[x][y] += 1;
            x = (x as isize + dx) as usize;
            y = (y as isize + dy) as usize;
        }
        board[x][y] += 1;
    }
    board.into_iter().flatten().filter(|&x| x > 1).count()
}

make_main! { 12 }
