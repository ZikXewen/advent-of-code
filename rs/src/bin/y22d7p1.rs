use std::collections::HashMap;

use aoc::make_main;
use itertools::Itertools;

#[derive(Default)]
struct Node {
    // For files, this is the size of the file, but negative.
    // For directories, this is the index of the directory in the arena.
    children: HashMap<String, isize>,
    parent: Option<usize>,
    size: usize,
}

fn compute(input: String) -> usize {
    let mut arena = vec![Node::default()];
    let mut cur = 0;
    let mut lines = input.lines().skip(1);
    while let Some(line) = lines.next() {
        let cmd = line.split_whitespace().collect_vec();
        match (cmd[0], cmd[1]) {
            ("$", "cd") => {
                if cmd[2] == ".." {
                    cur = arena[cur].parent.unwrap();
                } else if arena[cur].children.contains_key(cmd[2]) {
                    cur = arena[cur].children[cmd[2]] as usize;
                } else {
                    let id = arena.len();
                    arena.push(Node {
                        parent: Some(cur),
                        ..Default::default()
                    });
                    arena[cur].children.insert(cmd[2].to_string(), id as isize);
                    cur = id;
                }
            }
            (sz, _) => {
                if let Ok(sz) = sz.parse::<isize>() {
                    arena[cur].children.insert(cmd[1].to_string(), -sz);
                }
            }
        }
    }

    for i in (0..arena.len()).rev() {
        let mut sum = 0;
        for (_, &sz) in &arena[i].children {
            if sz < 0 {
                sum += (-sz) as usize;
            } else {
                sum += arena[sz as usize].size;
            }
        }
        arena[i].size = sum;
    }

    arena
        .iter()
        .filter_map(|node| {
            if node.size <= 100000 {
                Some(node.size)
            } else {
                None
            }
        })
        .sum()
}

make_main! { 95437 }
