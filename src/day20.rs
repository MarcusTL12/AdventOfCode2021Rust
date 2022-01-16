use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use itertools::Itertools;
use ndarray::Array2;

pub const PARTS: [fn(); 2] = [part1, part2];

fn parse_input(filename: &str) -> (Vec<bool>, Array2<bool>) {
    let mut lines = BufReader::new(File::open(filename).unwrap())
        .lines()
        .map(|l| l.unwrap());

    let rules = lines
        .next()
        .unwrap()
        .chars()
        .map(|c| match c {
            '.' => false,
            '#' => true,
            _ => unreachable!(),
        })
        .collect();

    assert!(lines.next().unwrap().is_empty());

    let mut img_buf = Vec::new();

    let mut w = 0;
    let mut h = 0;

    for l in lines {
        h += 1;
        w = l.len();
        img_buf.extend(l.chars().map(|c| match c {
            '.' => false,
            '#' => true,
            _ => unreachable!(),
        }));
    }

    (rules, Array2::from_shape_vec((h, w), img_buf).unwrap())
}

fn do_iters(rules: &[bool], img: &Array2<bool>, double_iters: usize) -> usize {
    let (h, w) = img.shape().iter().cloned().collect_tuple().unwrap();
    let mut img_a = Array2::from_elem(
        (
            h + 3 * 4 * (double_iters + 1),
            w + 3 * 4 * (double_iters + 1),
        ),
        false,
    );
    let mut img_b = Array2::from_elem(
        (
            h + 3 * 4 * (double_iters + 1),
            w + 3 * 4 * (double_iters + 1),
        ),
        rules[0],
    );

    let mut x_start = 3 * 2 * (double_iters + 1);
    let mut y_start = 3 * 2 * (double_iters + 1);
    let mut x_end = x_start + w;
    let mut y_end = y_start + h;

    for ((i, j), &b) in img.indexed_iter() {
        img_a[(i + y_start, j + x_start)] = b;
    }

    for _ in 0..double_iters {
        x_start -= 3;
        x_end += 3;
        y_start -= 3;
        y_end += 3;
        for (i, j) in (y_start..y_end).cartesian_product(x_start..x_end) {
            img_b[(i, j)] = rules[(-1..=1)
                .cartesian_product(-1..=1)
                .map(|(di, dj)| {
                    img_a[(
                        (i as isize + di) as usize,
                        (j as isize + dj) as usize,
                    )]
                })
                .fold(0, |n, b| (n << 1) + b as usize)];
        }
        x_start -= 3;
        x_end += 3;
        y_start -= 3;
        y_end += 3;
        for (i, j) in (y_start..y_end).cartesian_product(x_start..x_end) {
            img_a[(i, j)] = rules[(-1..=1)
                .cartesian_product(-1..=1)
                .map(|(di, dj)| {
                    img_b[(
                        (i as isize + di) as usize,
                        (j as isize + dj) as usize,
                    )]
                })
                .fold(0, |n, b| (n << 1) + b as usize)];
        }
    }

    img_a.into_iter().filter(|&x| x).count()
}

fn _show_img(img: &Array2<bool>) {
    let (h, w) = img.shape().iter().cloned().collect_tuple().unwrap();
    for i in 0..h {
        for j in 0..w {
            print!("{}", if img[(i, j)] { '#' } else { '.' });
        }
        println!();
    }
}

fn part1() {
    let (rules, img) = parse_input("input/day20/input");

    let ans = do_iters(&rules, &img, 1);

    println!("{ans}");
}

fn part2() {
    let (rules, img) = parse_input("input/day20/input");

    let ans = do_iters(&rules, &img, 25);

    println!("{ans}");
}
