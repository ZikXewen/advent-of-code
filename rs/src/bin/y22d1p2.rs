use aoc::make_main;

fn compute(input: String) -> String {
    let mut elves = input
        .split("\n\n")
        .map(|s| s.lines().map(|l| l.parse::<u32>().unwrap()).sum::<u32>())
        .collect::<Vec<_>>();
    elves.sort_by(|a, b| b.cmp(a));
    elves.iter().take(3).sum::<u32>().to_string()
}

make_main! { 45000 }
