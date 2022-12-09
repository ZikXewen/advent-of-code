use aoc::make_main;
use itertools::Itertools;

fn compute(input: String) -> isize {
    let mut vec = input
        .split(',')
        .map(|s| s.parse::<isize>().unwrap())
        .collect_vec();
    vec.sort();
    let md = vec[vec.len() / 2];
    vec.iter().map(|x| (x - md).abs()).sum()
}

make_main! { 37 }
