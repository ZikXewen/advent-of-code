use aoc::make_main;
use itertools::Itertools;

fn compute(input: String) -> usize {
    let mut ar = vec![0u8; 26];
    let mut over = 0;
    let chars = input.chars().collect_vec();
    for i in 0..14 {
        let c = chars[i] as usize - 'a' as usize;
        ar[c] += 1;
        if ar[c] == 2 {
            over += 1;
        }
    }
    for i in 14.. {
        if over == 0 {
            return i;
        }
        let c = chars[i - 14] as usize - 'a' as usize;
        ar[c] -= 1;
        if ar[c] == 1 {
            over -= 1;
        }
        let c = chars[i] as usize - 'a' as usize;
        ar[c] += 1;
        if ar[c] == 2 {
            over += 1;
        }
    }
    panic!("not found")
}

make_main! { 19 }
