use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use itertools::Itertools;

pub const PARTS: [fn(); 2] = [part1, part2];

fn part1() {
    let (depth, x) = BufReader::new(File::open("input/day2/input").unwrap())
        .lines()
        .map(|l| l.unwrap())
        .fold((0, 0), |(depth, x), l| {
            let (d, n) = l.split_ascii_whitespace().collect_tuple().unwrap();
            let n = n.parse::<i64>().unwrap();
            match d {
                "up" => (depth - n, x),
                "down" => (depth + n, x),
                "forward" => (depth, x + n),
                _ => unreachable!(),
            }
        });

    println!("{}", depth * x);
}

fn part2() {
    let (depth, x, _) = BufReader::new(File::open("input/day2/input").unwrap())
        .lines()
        .map(|l| l.unwrap())
        .fold((0, 0, 0), |(depth, x, aim), l| {
            let (d, n) = l.split_ascii_whitespace().collect_tuple().unwrap();
            let n = n.parse::<i64>().unwrap();
            match d {
                "up" => (depth, x, aim - n),
                "down" => (depth, x, aim + n),
                "forward" => (depth + n * aim, x + n, aim),
                _ => unreachable!(),
            }
        });

    println!("{}", depth * x);
}
