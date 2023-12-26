use std::collections::BTreeSet;

use aoc::make_main;

fn compute(input: String) -> usize {
    input
        .lines()
        .map(|s| {
            let (pat, s) = s.split_once(" | ").unwrap();
            let set = BTreeSet::from_iter(pat.split(' ').map(|s| BTreeSet::from_iter(s.chars())));
            let mut parser = vec![None; 10];
            parser[1] = Some(set.iter().find(|s| s.len() == 2).unwrap());
            parser[7] = Some(set.iter().find(|s| s.len() == 3).unwrap());
            parser[4] = Some(set.iter().find(|s| s.len() == 4).unwrap());
            parser[8] = Some(set.iter().find(|s| s.len() == 7).unwrap());
            parser[9] = Some(
                set.iter()
                    .find(|s| s.len() == 6 && s.is_superset(&parser[4].unwrap()))
                    .unwrap(),
            );
            parser[6] = Some(
                set.iter()
                    .find(|s| s.len() == 6 && !s.is_superset(&parser[7].unwrap()))
                    .unwrap(),
            );
            parser[0] = Some(
                set.iter()
                    .find(|s| s.len() == 6 && s != &parser[9].unwrap() && s != &parser[6].unwrap())
                    .unwrap(),
            );
            parser[3] = Some(
                set.iter()
                    .find(|s| s.len() == 5 && s.is_superset(&parser[7].unwrap()))
                    .unwrap(),
            );
            parser[5] = Some(
                set.iter()
                    .find(|s| s.len() == 5 && s.is_subset(&parser[6].unwrap()))
                    .unwrap(),
            );
            parser[2] = Some(
                set.iter()
                    .find(|s| s.len() == 5 && s != &parser[5].unwrap() && s != &parser[3].unwrap())
                    .unwrap(),
            );
            s.split(' ').fold(0, |sm, cr| {
                let cr = BTreeSet::from_iter(cr.chars());
                10 * sm + (0..=9).find(|i| parser[*i] == Some(&cr)).unwrap()
            })
        })
        .sum()
}

make_main! { 61229 }
