use aoc::make_main;
use itertools::Itertools;

fn compute(input: String) -> String {
    input
        .lines()
        .tuple_windows()
        .map(|(a, b)| b.parse::<u32>().unwrap() > a.parse().unwrap())
        .filter(|&x| x)
        .count()
        .to_string()
}

make_main! { 7 }
