use std::fs::read_to_string;

use once_cell::sync::Lazy;
use regex::Regex;

pub const PARTS: [fn(); 2] = [part1, part2];

static REG: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
        r"inp w
mul x 0
add x z
mod x 26
div z (1|26)
add x (-?\d+)
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y (-?\d+)
mul y x
add z y",
    )
    .unwrap()
});

fn get_parameters(filename: &str) -> Vec<(bool, i64, i64)> {
    let s = read_to_string(filename).unwrap();

    REG.captures_iter(&s)
        .map(|c| (&c[1] == "26", c[2].parse().unwrap(), c[3].parse().unwrap()))
        .collect()
}

fn rec<I: Iterator<Item = i64> + Clone>(
    z: i64,
    params: &[(bool, i64, i64)],
    n: i64,
    ds: I,
) -> Option<i64> {
    if let Some(&(a, b, c)) = params.get(0) {
        if a {
            let d = z % 26 + b;
            if 1 <= d && d <= 9 {
                rec(z / 26, &params[1..], 10 * n + d, ds)
            } else {
                None
            }
        } else {
            ds.clone().find_map(|d| {
                rec(26 * z + c + d, &params[1..], 10 * n + d, ds.clone())
            })
        }
    } else if z == 0 {
        Some(n)
    } else {
        None
    }
}

fn part1() {
    let params = get_parameters("input/day24/input");

    let ans = rec(0, &params, 0, (1..=9).rev()).unwrap();

    println!("{ans}");
}

fn part2() {
    let params = get_parameters("input/day24/input");

    let ans = rec(0, &params, 0, 1..=9).unwrap();

    println!("{ans}");
}
