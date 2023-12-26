use aoc::make_main;
use itertools::Itertools;

fn compute(input: String) -> String {
    input
        .lines()
        .tuple_windows()
        .map(|(a, b, c)| {
            a.parse::<u32>().unwrap() + b.parse::<u32>().unwrap() + c.parse::<u32>().unwrap()
        })
        .tuple_windows()
        .map(|(a, b)| b > a)
        .filter(|&x| x)
        .count()
        .to_string()
}

make_main! { 5 }
