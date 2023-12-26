use aoc::make_main;
use itertools::Itertools;

fn compute(input: String) -> usize {
    let chars = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as isize)
                .collect_vec()
        })
        .collect_vec();
    let mut scores = vec![vec![0; chars[0].len()]; chars.len()];
    for i in 1..(chars.len() - 1) {
        for j in 1..(chars[i].len() - 1) {
            let mut to_b = 1;
            while i + to_b < chars.len() - 1 && chars[i + to_b][j] < chars[i][j] {
                to_b += 1;
            }
            let mut to_t = 1;
            while i - to_t > 0 && chars[i - to_t][j] < chars[i][j] {
                to_t += 1;
            }
            let mut to_r = 1;
            while j + to_r < chars[i].len() - 1 && chars[i][j + to_r] < chars[i][j] {
                to_r += 1;
            }
            let mut to_l = 1;
            while j - to_l > 0 && chars[i][j - to_l] < chars[i][j] {
                to_l += 1;
            }
            scores[i][j] = to_r * to_l * to_t * to_b;
        }
    }
    *scores.iter().flatten().max().unwrap()
}

make_main! { 8 }
