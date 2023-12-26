use aoc::make_main;

fn compute(input: String) -> u32 {
    input
        .lines()
        .map(|line| match line {
            "B X" => 1,
            "C X" => 2,
            "A X" => 3,
            "A Y" => 4,
            "B Y" => 5,
            "C Y" => 6,
            "C Z" => 7,
            "A Z" => 8,
            "B Z" => 9,
            _ => 0,
        })
        .sum()
}

make_main! { 12 }
