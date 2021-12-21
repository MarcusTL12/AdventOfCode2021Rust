use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use arrayvec::ArrayVec;
use hashbrown::HashMap;
use itertools::Itertools;

pub const PARTS: [fn(); 2] = [part1, part2];

fn part1() {
    let mut pos = BufReader::new(File::open("input/day21/input").unwrap())
        .lines()
        .map(|l| l.unwrap())
        .map(|l| l.split(' ').last().unwrap().parse::<usize>().unwrap())
        .collect::<ArrayVec<_, 2>>()
        .into_inner()
        .unwrap();

    let mut score = [0, 0];

    let inds = [1, 0];

    let mut die = (1..=100).cycle();

    let mut i = 0;
    let mut amt_rolls = 0;

    while score.iter().all(|&s| s < 1000) {
        amt_rolls += 3;
        let n: usize = (&mut die).take(3).sum();
        pos[i] = (pos[i] + n - 1) % 10 + 1;
        score[i] += pos[i];
        i = inds[i];
    }

    let ans = score.iter().min().unwrap() * amt_rolls;

    println!("{}", ans);
}

fn rec(
    k: (usize, usize, usize, usize, bool),
    memo: &mut HashMap<(usize, usize, usize, usize, bool), (usize, usize)>,
) -> (usize, usize) {
    if let Some(&x) = memo.get(&k) {
        return x;
    } else {
        let ans = if k.2 < 21 && k.3 < 21 {
            let mut acc = (0, 0);

            for diesum in (1..=3)
                .cartesian_product(1..=3)
                .cartesian_product(1..=3)
                .map(|((a, b), c)| a + b + c)
            {
                let n_pos_a = if !k.4 {
                    (k.0 + diesum - 1) % 10 + 1
                } else {
                    k.0
                };
                let n_pos_b = if k.4 {
                    (k.1 + diesum - 1) % 10 + 1
                } else {
                    k.1
                };
                let n_score_a = k.2 + if !k.4 { n_pos_a } else { 0 };
                let n_score_b = k.3 + if k.4 { n_pos_b } else { 0 };
                let (da, db) =
                    rec((n_pos_a, n_pos_b, n_score_a, n_score_b, !k.4), memo);
                acc.0 += da;
                acc.1 += db;
            }

            acc
        } else if k.2 < 21 {
            (0, 1)
        } else if k.3 < 21 {
            (1, 0)
        } else {
            unreachable!()
        };
        memo.insert(k, ans);
        ans
    }
}

fn part2() {
    let (pos_a, pos_b) =
        BufReader::new(File::open("input/day21/input").unwrap())
            .lines()
            .map(|l| l.unwrap())
            .map(|l| l.split(' ').last().unwrap().parse::<usize>().unwrap())
            .collect_tuple()
            .unwrap();

    let (a, b) = rec((pos_a, pos_b, 0, 0, false), &mut HashMap::new());

    let ans = [a, b].iter().cloned().max().unwrap();

    println!("{}", ans);
}
