use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use itertools::{Itertools, MinMaxResult};

pub const PARTS: [fn(); 2] = [part1, part2];

fn parse_input(filename: &str) -> Vec<i64> {
    BufReader::new(File::open(filename).unwrap())
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect()
}

fn part1() {
    let inp = parse_input("input/day7/input");

    let ans = match inp.iter().minmax() {
        MinMaxResult::MinMax(&min, &max) => (min..=max)
            .map(|x| inp.iter().map(|n| (x - n).abs()).sum::<i64>())
            .min()
            .unwrap(),
        _ => 0,
    };

    println!("{}", ans);
}

fn part2() {
    let inp = parse_input("input/day7/input");

    let ans = match inp.iter().minmax() {
        MinMaxResult::MinMax(&min, &max) => (min..=max)
            .map(|x| {
                inp.iter()
                    .map(|n| (x - n).abs())
                    .map(|n| (n * (n + 1)) / 2)
                    .sum::<i64>()
            })
            .min()
            .unwrap(),
        _ => 0,
    };

    println!("{}", ans);
}
