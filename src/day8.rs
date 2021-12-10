use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

use itertools::Itertools;

pub const PARTS: [fn(); 2] = [part1, part2];

fn part1() {
    let ans: usize = BufReader::new(File::open("input/day8/input").unwrap())
        .lines()
        .map(|l| l.unwrap())
        .map(|l| {
            l.split('|')
                .skip(1)
                .next()
                .unwrap()
                .split(' ')
                .filter(|s| [2, 3, 4, 7].iter().any(|&l| s.len() == l))
                .count()
        })
        .sum();

    println!("{}", ans);
}

fn part2() {
    let valid_segments: HashMap<_, _> = [
        [true, true, true, false, true, true, true],
        [false, false, true, false, false, true, false],
        [true, false, true, true, true, false, true],
        [true, false, true, true, false, true, true],
        [false, true, true, true, false, true, false],
        [true, true, false, true, false, true, true],
        [true, true, false, true, true, true, true],
        [true, false, true, false, false, true, false],
        [true, true, true, true, true, true, true],
        [true, true, true, true, false, true, true],
    ]
    .iter()
    .enumerate()
    .map(|(a, b)| (b, a))
    .collect();

    let ans: usize = BufReader::new(File::open("input/day8/input").unwrap())
        .lines()
        .map(|l| l.unwrap())
        .map(|l| {
            let (first, second) = l
                .split('|')
                .map(|p| p.split_ascii_whitespace())
                .collect_tuple()
                .unwrap();

            let correct_perm = (0..7)
                .permutations(7)
                .find(|perm| {
                    first.clone().all(|p| {
                        let mut segments = [false; 7];
                        for c in p.chars() {
                            segments[perm[(c as u8 - b'a') as usize]] = true;
                        }
                        valid_segments.contains_key(&segments)
                    })
                })
                .unwrap();

            second.fold(0, |n, p| {
                let mut segments = [false; 7];
                for c in p.chars() {
                    segments[correct_perm[(c as u8 - b'a') as usize]] = true;
                }
                10 * n + valid_segments[&segments]
            })
        })
        .sum();

    println!("{}", ans);
}
