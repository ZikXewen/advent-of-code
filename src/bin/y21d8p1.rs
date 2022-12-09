use aoc::make_main;

fn compute(input: String) -> usize {
    let un = [2, 4, 3, 7];
    input
        .lines()
        .map(|s| {
            s.split_once(" | ")
                .unwrap()
                .1
                .split(' ')
                .filter(|s| un.contains(&s.len()))
                .count()
        })
        .sum()
}

make_main! { 26 }
