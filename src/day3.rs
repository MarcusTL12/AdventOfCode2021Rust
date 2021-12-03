use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub const PARTS: [fn(); 2] = [part1, part2];

fn part1() {
    let mut bits = 0;

    let inp: Vec<_> = BufReader::new(File::open("input/day3/input").unwrap())
        .lines()
        .map(|l| l.unwrap())
        .map(|l| {
            bits = l.len();
            i32::from_str_radix(&l, 2).unwrap()
        })
        .collect();

    let counts = inp.iter().fold(vec![0usize; bits], |mut c, n| {
        let mut n = *n;
        for i in 0..bits {
            c[i] += (n & 1) as usize;
            n >>= 1;
        }

        c
    });

    let g = counts
        .into_iter()
        .rev()
        .map(|c| 2 * c > inp.len())
        .fold(0, |n, c| (n << 1) | if c { 1 } else { 0 });

    let mask = (0..bits).fold(0, |m, _| (m << 1) | 1);

    let ans = g * (!g & mask);

    println!("{}", ans);
}

fn part2() {}
