use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

use itertools::Itertools;
use once_cell::sync::Lazy;
use regex::Regex;

static REG: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(\d+),(\d+) -> (\d+),(\d+)").unwrap());

pub const PARTS: [fn(); 2] = [part1, part2];

fn part1() {
    let mut points = HashMap::new();

    for (x1, y1, x2, y2) in
        BufReader::new(File::open("input/day5/input").unwrap())
            .lines()
            .map(|l| l.unwrap())
            .map(|l| {
                if let Some(c) = REG.captures(&l) {
                    (1..=4)
                        .map(|i| c[i].parse::<i64>().unwrap())
                        .collect_tuple()
                        .unwrap()
                } else {
                    unreachable!()
                }
            })
            .filter(|(x1, y1, x2, y2)| x1 == x2 || y1 == y2)
    {
        let dx = (x2 - x1).signum();
        let dy = (y2 - y1).signum();

        let mut x = x1 - dx;
        let mut y = y1 - dy;

        while (x, y) != (x2, y2) {
            x += dx;
            y += dy;

            if let Some(n) = points.get_mut(&(x, y)) {
                *n += 1;
            } else {
                points.insert((x, y), 1usize);
            }
        }
    }

    let ans = points.values().filter(|&&x| x >= 2).count();

    println!("{}", ans);
}

fn part2() {
    let mut points = HashMap::new();

    for (x1, y1, x2, y2) in
        BufReader::new(File::open("input/day5/input").unwrap())
            .lines()
            .map(|l| l.unwrap())
            .map(|l| {
                if let Some(c) = REG.captures(&l) {
                    (1..=4)
                        .map(|i| c[i].parse::<i64>().unwrap())
                        .collect_tuple()
                        .unwrap()
                } else {
                    unreachable!()
                }
            })
    {
        let dx = (x2 - x1).signum();
        let dy = (y2 - y1).signum();

        let mut x = x1 - dx;
        let mut y = y1 - dy;

        while (x, y) != (x2, y2) {
            x += dx;
            y += dy;

            if let Some(n) = points.get_mut(&(x, y)) {
                *n += 1;
            } else {
                points.insert((x, y), 1usize);
            }
        }
    }

    let ans = points.values().filter(|&&x| x >= 2).count();

    println!("{}", ans);
}
