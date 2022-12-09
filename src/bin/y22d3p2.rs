use std::collections::HashSet;

use aoc::make_main;
use itertools::Itertools;

fn compute(input: String) -> usize {
    input
        .lines()
        .chunks(3)
        .into_iter()
        .map(|s| {
            let lines = s.collect_vec();
            let set1 = HashSet::<char>::from_iter(lines[0].chars());
            let set2 = HashSet::<char>::from_iter(lines[1].chars().filter(|c| set1.contains(&c)));
            let ch = lines[2].chars().find(|c| set2.contains(&c)).unwrap();
            if ('a'..='z').contains(&ch) {
                ch as usize - 'a' as usize + 1
            } else {
                ch as usize - 'A' as usize + 27
            }
        })
        .sum()
}

make_main! { 70 }
