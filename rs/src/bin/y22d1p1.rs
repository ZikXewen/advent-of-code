use aoc::make_main;

fn compute(input: String) -> String {
    input
        .split("\n\n")
        .map(|s| s.lines().map(|l| l.parse::<u32>().unwrap()).sum::<u32>())
        .max()
        .unwrap()
        .to_string()
}

make_main! { 24000 }
