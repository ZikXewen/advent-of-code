use aoc::make_main;

fn compute(input: String) -> u32 {
    input
        .lines()
        .map(|line| match line {
            "B X" => 1,
            "C Y" => 2,
            "A Z" => 3,
            "A X" => 4,
            "B Y" => 5,
            "C Z" => 6,
            "C X" => 7,
            "A Y" => 8,
            "B Z" => 9,
            _ => 0,
        })
        .sum()
}

make_main! { 15 }
