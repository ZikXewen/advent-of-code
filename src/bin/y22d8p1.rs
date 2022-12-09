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
    let mut marked = vec![vec![false; chars[0].len()]; chars.len()];
    for i in 0..chars.len() {
        let mut ml = -1;
        for fl in 0..chars[i].len() {
            if chars[i][fl] > ml {
                marked[i][fl] = true;
                ml = chars[i][fl];
            }
        }
        let mut mr = -1;
        for fr in (0..chars[i].len()).rev() {
            if chars[i][fr] > mr {
                marked[i][fr] = true;
                mr = chars[i][fr];
            }
        }
    }
    for i in 0..chars[0].len() {
        let mut mt = -1;
        for ft in 0..chars.len() {
            if chars[ft][i] > mt {
                marked[ft][i] = true;
                mt = chars[ft][i];
            }
        }
        let mut mb = -1;
        for fb in (0..chars.len()).rev() {
            if chars[fb][i] > mb {
                marked[fb][i] = true;
                mb = chars[fb][i];
            }
        }
    }
    // dbg!(&marked);
    marked.iter().flatten().filter(|&&x| x).count()
}

make_main! { 21 }
