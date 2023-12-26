use std::collections::HashSet;

use aoc::make_main;

fn compute(input: String) -> usize {
    input
        .lines()
        .map(|s| {
            let (a, b) = s.split_at(s.len() / 2);
            let map = HashSet::<char>::from_iter(a.chars());
            let ch = b.chars().find(|&c| map.get(&c).is_some()).unwrap();
            if ('a'..='z').contains(&ch) {
                ch as usize - 'a' as usize + 1
            } else {
                ch as usize - 'A' as usize + 27
            }
        })
        .sum()
}

make_main! { 157 }
