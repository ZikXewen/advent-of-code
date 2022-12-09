use aoc::make_main;
use itertools::Itertools;

fn compute(input: String) -> String {
    let (init, moves) = input.split_once("\n\n").unwrap();
    let mut init = init.lines().rev();
    let len = init
        .next()
        .unwrap()
        .trim()
        .split_whitespace()
        .last()
        .unwrap()
        .parse::<usize>()
        .unwrap();
    let mut crates = vec![vec![]; len];
    for lvl in init {
        let lvl = lvl.chars().collect_vec();
        for i in 0.. {
            let po = 4 * i + 1;
            if po >= lvl.len() {
                break;
            }
            if !lvl[po].is_whitespace() {
                crates[i].push(lvl[po]);
            }
        }
    }
    for m in moves.lines() {
        let m = m.split_whitespace().collect_vec();
        let n = m[1].parse::<usize>().unwrap();
        let from = m[3].parse::<usize>().unwrap() - 1;
        let to = m[5].parse::<usize>().unwrap() - 1;
        let n = (crates[from].len() - n)..;
        let drained = crates[from].drain(n).collect_vec();
        crates[to].extend(drained);
    }
    crates.iter().map(|c| c.last().unwrap()).join("")
}

make_main! { MCD }
