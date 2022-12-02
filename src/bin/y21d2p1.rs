use aoc::make_main;

fn compute(input: String) -> i32 {
    let (x, y) = input.lines().fold((0, 0), |(mut x, mut y), s| {
        let (ty, ct) = s.split_once(' ').unwrap();
        let ct = ct.parse::<i32>().unwrap();
        match ty {
            "forward" => x += ct,
            "up" => y += ct,
            "down" => y -= ct,
            _ => (),
        };
        (x, y)
    });
    (x * y).abs()
}

make_main! { 150 }
