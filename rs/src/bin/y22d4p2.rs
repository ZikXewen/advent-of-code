use aoc::make_main;

fn compute(input: String) -> usize {
    input
        .lines()
        .filter(|line| {
            let (a, b) = line.split_once(',').unwrap();
            let (a1, a2) = a.split_once('-').unwrap();
            let (b1, b2) = b.split_once('-').unwrap();
            let a1 = a1.parse::<usize>().unwrap();
            let a2 = a2.parse::<usize>().unwrap();
            let b1 = b1.parse::<usize>().unwrap();
            let b2 = b2.parse::<usize>().unwrap();

            (a1 <= b1 && b1 <= a2) || (b1 <= a1 && a1 <= b2)
        })
        .count()
}

make_main! { 4 }
