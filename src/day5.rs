use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use itertools::Itertools;
use ndarray::Array2;
use once_cell::sync::Lazy;
use regex::Regex;

static REG: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(\d+),(\d+) -> (\d+),(\d+)").unwrap());

pub const PARTS: [fn(); 2] = [part1, part2];

fn part1() {
    let mut points = Array2::from_elem((1000, 1000), 0u8);

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

            match points.get_mut((x as usize, y as usize)) {
                Some(x) if *x == 0 => *x = 1,
                Some(x) if *x == 1 => *x = 2,
                Some(2) => {}
                _ => unreachable!(),
            }
        }
    }

    let ans = points.iter().filter(|&&x| x == 2).count();

    println!("{ans}");
}

fn part2() {
    let mut points = Array2::from_elem((1000, 1000), 0u8);

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

            match points.get_mut((x as usize, y as usize)) {
                Some(x) if *x == 0 => *x = 1,
                Some(x) if *x == 1 => *x = 2,
                Some(2) => {}
                _ => unreachable!(),
            }
        }
    }

    let ans = points.iter().filter(|&&x| x == 2).count();

    println!("{ans}");
}
