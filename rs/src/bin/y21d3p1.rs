use aoc::make_main;

fn compute(input: String) -> i32 {
    let len = input.lines().next().unwrap().len();
    let mx = input
        .lines()
        .fold(vec![0; len], |acc, line| {
            acc.iter()
                .zip(line.chars())
                .map(|(a, b)| a + if b == '1' { 1 } else { -1 })
                .collect()
        })
        .iter()
        .fold(0, |acc, &x| 2 * acc + if x > 0 { 1 } else { 0 });
    mx * ((1 << len) - mx - 1)
}

make_main! { 198 }
