use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use itertools::Itertools;

pub const PARTS: [fn(); 2] = [part1, part2];

fn part1() {
    let ans = BufReader::new(File::open("input/day1/input").unwrap())
        .lines()
        .map(|l| l.unwrap().parse::<u64>().unwrap())
        .tuple_windows()
        .filter(|(a, b)| a < b)
        .count();

    println!("{ans}");
}

fn part2() {
    let ans = BufReader::new(File::open("input/day1/input").unwrap())
        .lines()
        .map(|l| l.unwrap().parse::<u64>().unwrap())
        .tuple_windows()
        .map(|(a, b, c)| a + b + c)
        .tuple_windows()
        .filter(|(a, b)| a < b)
        .count();

    println!("{ans}");
}
