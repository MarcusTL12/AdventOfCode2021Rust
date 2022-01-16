use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

use arrayvec::ArrayVec;
use itertools::Itertools;

pub const PARTS: [fn(); 2] = [part1, part2];

fn parse_input(filename: &str) -> (Vec<[usize; 2]>, Vec<(usize, usize)>) {
    let mut coords = Vec::new();
    let mut folds = Vec::new();

    let mut lines = BufReader::new(File::open(filename).unwrap())
        .lines()
        .map(|l| l.unwrap());

    for l in &mut lines {
        if l.is_empty() {
            break;
        }
        coords.push(
            l.split(',')
                .map(|s| s.parse().unwrap())
                .collect::<ArrayVec<_, 2>>()
                .into_inner()
                .unwrap(),
        );
    }

    for l in lines {
        let s = l.split_ascii_whitespace().skip(2).next().unwrap();
        let (axis, n) = s.split('=').collect_tuple().unwrap();
        folds.push((
            match axis {
                "x" => 0,
                "y" => 1,
                _ => unreachable!(),
            },
            n.parse().unwrap(),
        ))
    }

    (coords, folds)
}

fn part1() {
    let (mut coords, folds) = parse_input("input/day13/input");

    let (cn, n) = folds[0];
    for coord in coords.iter_mut() {
        if coord[cn] > n {
            coord[cn] = 2 * n - coord[cn];
        }
    }

    let ans = coords.into_iter().collect::<HashSet<_>>().len();

    println!("{ans}");
}

fn part2() {
    let (mut coords, folds) = parse_input("input/day13/input");

    for (cn, n) in folds {
        for coord in coords.iter_mut() {
            if coord[cn] > n {
                coord[cn] = 2 * n - coord[cn];
            }
        }
    }

    let coords: HashSet<_> = coords.into_iter().collect();

    let maxx = coords.iter().map(|c| c[0]).max().unwrap();
    let maxy = coords.iter().map(|c| c[1]).max().unwrap();

    for y in 0..=maxy {
        for x in 0..=maxx {
            print!(
                "{}",
                if coords.contains(&[x, y]) {
                    "██"
                } else {
                    "  "
                }
            );
        }
        println!();
    }
}
