use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub const PARTS: [fn(); 2] = [part1, part2];

fn parse_input(filename: &str) -> (usize, Vec<i32>) {
    let mut bits = 0;

    let inp: Vec<_> = BufReader::new(File::open(filename).unwrap())
        .lines()
        .map(|l| l.unwrap())
        .map(|l| {
            bits = l.len();
            i32::from_str_radix(&l, 2).unwrap()
        })
        .collect();

    (bits, inp)
}

fn part1() {
    let (bits, inp) = parse_input("input/day3/input");

    let g = inp
        .iter()
        .fold(vec![0usize; bits], |mut c, n| {
            let mut n = *n;
            for i in 0..bits {
                c[i] += (n & 1) as usize;
                n >>= 1;
            }

            c
        })
        .into_iter()
        .rev()
        .map(|c| 2 * c > inp.len())
        .fold(0, |n, c| (n << 1) | if c { 1 } else { 0 });

    let mask = (0..bits).fold(0, |m, _| (m << 1) | 1);

    let ans = g * (!g & mask);

    println!("{ans}");
}

fn part2() {
    let (bits, inp) = parse_input("input/day3/input");

    let ox = (0..bits).rev().fold(inp.clone(), |mut inp, i| {
        if inp.len() > 1 {
            let b =
                2 * inp.iter().map(|n| ((n >> i) & 1) as usize).sum::<usize>()
                    >= inp.len();

            inp.retain(|x| if ((x >> i) & 1) != 0 { b } else { !b });
        }
        inp
    })[0];

    let co2 = (0..bits).rev().fold(inp.clone(), |mut inp, i| {
        if inp.len() > 1 {
            let b =
                2 * inp.iter().map(|n| ((n >> i) & 1) as usize).sum::<usize>()
                    >= inp.len();

            inp.retain(|x| if ((x >> i) & 1) != 0 { !b } else { b });
        }
        inp
    })[0];

    let ans = ox * co2;

    println!("{ans}");
}
