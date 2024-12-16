extern crate regex;
use regex::Regex;

fn main() {
    let input = include_str!("../day-13.data");
    let button = Regex::new(r"X\+(?P<x>\d+),\sY\+(?P<y>\d+)").unwrap();
    let prize = Regex::new(r"X=(?P<x>\d+),\sY=(?P<y>\d+)").unwrap();

    let now = std::time::Instant::now();
    let machines: isize = input
        .split("\n\n")
        .map(|chunk| {
            let lines: Vec<_> = chunk.lines().collect();
            let parse_coords = |line: &str, re: &Regex| {
                let caps = re.captures(line).unwrap();
                let x = caps.name("x").unwrap().as_str().parse::<isize>().unwrap();
                let y = caps.name("y").unwrap().as_str().parse::<isize>().unwrap();
                (x, y)
            };
            let a_coords = parse_coords(&lines[0], &button);
            let b_coords = parse_coords(&lines[1], &button);
            let prize_coords = parse_coords(&lines[2], &prize);
            (a_coords, b_coords, prize_coords)
        })
        .map(|machine| {
            let (a, b, c) = machine;
            cramer(a, b, c, 10000000000000)
        })
        .filter_map(|x| x)
        .sum();

    println!("Tokens: {:?} Elapsed: {:?}", machines, now.elapsed());
}

// x = c_0 b_1 - b_0 c_1 / a_0 b_1 - b_0 a_1
// y = a_0 c_1 - c_0 a_1 / a_0 b_1 - b_0 a_1
fn cramer(
    a: (isize, isize),
    b: (isize, isize),
    mut c: (isize, isize),
    scale: isize,
) -> Option<isize> {
    c.0 += scale;
    c.1 += scale;

    let d = a.0 * b.1 - b.0 * a.1;
    if d == 0 {
        // Colinear, no solution
        return None;
    }

    let d_x = c.0 * b.1 - b.0 * c.1;
    let d_y = a.0 * c.1 - c.0 * a.1;

    if (d_x % d) != 0 || (d_y % d) != 0 {
        // Can't press a button fractionally
        return None;
    }

    Some((d_x / d) * 3 + d_y / d)
}
