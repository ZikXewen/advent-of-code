use aoc::make_main;
use itertools::Itertools;

fn compute(input: String) -> usize {
    let mut v = input
        .split(',')
        .map(|s| s.parse::<usize>().unwrap())
        .collect_vec();
    for i in 0.. {
        if i >= v.len() {
            return i;
        }
        for j in (v[i]..80).step_by(7) {
            v.push(j + 9);
        }
    }
    unreachable!()
}

make_main! { 5934 }
