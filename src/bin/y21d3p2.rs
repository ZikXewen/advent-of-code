use aoc::make_main;

fn sub_compute(mut chars: Vec<Vec<char>>, keep: char) -> u32 {
    for i in 0..chars[0].len() {
        let ones = chars.iter().filter(|x| x[i] == '1').count();
        if ones * 2 >= chars.len() {
            chars = chars.into_iter().filter(|x| x[i] == keep).collect();
        } else {
            chars = chars.into_iter().filter(|x| x[i] != keep).collect();
        }
        if chars.len() == 1 {
            return u32::from_str_radix(&chars[0].iter().collect::<String>(), 2).unwrap();
        }
    }
    panic!("No solution found");
}

fn compute(input: String) -> u32 {
    let chars: Vec<Vec<_>> = input.lines().map(|x| x.chars().collect()).collect();
    sub_compute(chars.clone(), '1') * sub_compute(chars, '0')
}

make_main! { 230 }
