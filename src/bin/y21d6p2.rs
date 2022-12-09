use aoc::make_main;

fn compute(input: String) -> usize {
    let mut mem = vec![1; 256 + 10];
    for i in (0..256).rev() {
        for j in (i..256).step_by(7) {
            mem[i] += mem[j + 9];
        }
    }
    input
        .split(',')
        .map(|s| mem[s.parse::<usize>().unwrap()])
        .sum()
}

make_main! { 26984457539 }
