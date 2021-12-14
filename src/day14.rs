use std::{
    fs::File,
    io::{BufRead, BufReader},
    mem::swap,
};

use itertools::{Itertools, MinMaxResult};
use ndarray::Array2;

pub const PARTS: [fn(); 2] = [part1, part2];

fn parse_input(filename: &str) -> (Array2<usize>, Array2<u8>, u8) {
    let mut lines = BufReader::new(File::open(filename).unwrap())
        .lines()
        .map(|l| l.unwrap());

    let mut init = Array2::zeros((26, 26));

    let firstline = lines.next().unwrap();

    for (a, b) in firstline.chars().zip(firstline.chars().skip(1)) {
        let a = (a as u8 - b'A') as usize;
        let b = (b as u8 - b'A') as usize;
        init[(a, b)] += 1;
    }

    lines.next().unwrap();

    let mut insertions = Array2::zeros((26, 26));

    for l in lines {
        let (a, b) = l.split(" -> ").collect_tuple().unwrap();
        let (i, j) = a
            .chars()
            .map(|c| (c as u8 - b'A') as usize)
            .collect_tuple()
            .unwrap();
        insertions[(i, j)] = b.chars().next().unwrap() as u8 - b'A';
    }

    (
        init,
        insertions,
        firstline.chars().last().unwrap() as u8 - b'A',
    )
}

fn do_step(
    insertions: &Array2<u8>,
    counts: &Array2<usize>,
    new_counts: &mut Array2<usize>,
) {
    new_counts.fill(0);

    for ((i, j), &c) in insertions.indexed_iter() {
        if c != 0 {
            new_counts[(i, c as usize)] += counts[(i, j)];
            new_counts[(c as usize, j)] += counts[(i, j)];
        } else {
            new_counts[(i, j)] = counts[(i, j)];
        }
    }
}

fn part1() {
    let (mut counts1, insertions, lastchar) = parse_input("input/day14/input");
    let mut counts2 = counts1.clone();

    for _ in 0..10 {
        do_step(&insertions, &counts1, &mut counts2);
        swap(&mut counts1, &mut counts2);
    }

    let mut char_counts: Vec<_> =
        counts1.rows().into_iter().map(|row| row.sum()).collect();

    char_counts[lastchar as usize] += 1;

    let ans = match char_counts.into_iter().filter(|&x| x != 0).minmax() {
        MinMaxResult::MinMax(a, b) => b - a,
        _ => 0,
    };

    println!("{}", ans);
}

fn part2() {
    let (mut counts1, insertions, lastchar) = parse_input("input/day14/input");
    let mut counts2 = counts1.clone();

    for _ in 0..40 {
        do_step(&insertions, &counts1, &mut counts2);
        swap(&mut counts1, &mut counts2);
    }

    let mut char_counts: Vec<_> =
        counts1.rows().into_iter().map(|row| row.sum()).collect();

    char_counts[lastchar as usize] += 1;

    let ans = match char_counts.into_iter().filter(|&x| x != 0).minmax() {
        MinMaxResult::MinMax(a, b) => b - a,
        _ => 0,
    };

    println!("{}", ans);
}
