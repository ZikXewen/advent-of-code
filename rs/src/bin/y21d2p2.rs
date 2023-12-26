use aoc::make_main;

fn compute(input: String) -> i32 {
    let mut x = 0;
    let mut y = 0;
    let mut aim = 0;
    for s in input.lines() {
        let (ty, ct) = s.split_once(' ').unwrap();
        let ct = ct.parse::<i32>().unwrap();
        match ty {
            "forward" => {
                x += ct;
                y += ct * aim;
            }
            "up" => aim += ct,
            "down" => aim -= ct,
            _ => (),
        };
    }
    (x * y).abs()
}

make_main! { 900 }
