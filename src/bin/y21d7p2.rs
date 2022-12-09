use aoc::make_main;
use itertools::Itertools;

fn compute(input: String) -> isize {
    let mut vec = input
        .split(',')
        .map(|s| s.parse::<isize>().unwrap())
        .collect_vec();
    vec.sort();
    let mut sm = vec.iter().sum::<isize>();
    let mut cr = vec.iter().map(|i| i * (i + 1) / 2).sum::<isize>();
    let mut mn = cr;
    let mut k = 0;
    for i in 0..=vec.last().copied().unwrap() {
        while k < vec.len() && vec[k] == i {
            k += 1;
        }
        mn = mn.min(cr);
        cr += k as isize - sm;
        sm -= vec.len() as isize;
    }
    mn
}

make_main! { 168 }
