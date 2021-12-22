use std::{
    cmp::{max, min},
    fs::File,
    io::{BufRead, BufReader}, mem,
};

use arrayvec::ArrayVec;
use hashbrown::HashMap;
use itertools::Itertools;
use once_cell::sync::Lazy;
use regex::Regex;

static REG: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
        r"(on|off) x=(-?\d+)..(-?\d+),y=(-?\d+)..(-?\d+),z=(-?\d+)..(-?\d+)",
    )
    .unwrap()
});

pub const PARTS: [fn(); 2] = [part1, part2];

fn part1() {
    let ans = BufReader::new(File::open("input/day22/input").unwrap())
        .lines()
        .map(|l| l.unwrap())
        .map(|l| {
            if let Some(c) = REG.captures(&l) {
                (
                    match &c[1] {
                        "on" => true,
                        "off" => false,
                        _ => unreachable!(),
                    },
                    (2..=7)
                        .map(|i| c[i].parse().unwrap())
                        .collect::<ArrayVec<i64, 6>>()
                        .into_inner()
                        .unwrap(),
                )
            } else {
                unreachable!()
            }
        })
        .take_while(|(_, x)| x.iter().all(|x| x.abs() <= 50))
        .flat_map(|(state, curbox)| {
            (curbox[0]..=curbox[1])
                .cartesian_product(curbox[2]..=curbox[3])
                .cartesian_product(curbox[4]..=curbox[5])
                .map(move |((x, y), z)| (state, (x, y, z)))
        })
        .fold(HashMap::new(), |mut screen, (state, pos)| {
            if let Some(x) = screen.get_mut(&pos) {
                *x = state;
            } else {
                screen.insert(pos, state);
            }
            screen
        })
        .values()
        .filter(|&&x| x)
        .count();

    println!("{}", ans);
}

type BoxType = [i64; 6];

fn is_any_overlap(a: &BoxType, b: &BoxType) -> bool {
    !a.iter()
        .zip(b.iter())
        .tuples()
        .any(|((a_lo, b_lo), (a_hi, b_hi))| a_lo > b_hi || a_hi < b_lo)
}

fn get_overlap(a: &BoxType, b: &BoxType) -> Option<[i64; 6]> {
    if is_any_overlap(a, b) {
        Some(
            a.iter()
                .zip(b.iter())
                .tuples()
                .flat_map(|((&a_lo, &b_lo), (&a_hi, &b_hi))| {
                    [max(a_lo, b_lo), min(a_hi, b_hi)].into_iter()
                })
                .collect::<ArrayVec<_, 6>>()
                .into_inner()
                .unwrap(),
        )
    } else {
        None
    }
}

fn box_diff(a: &BoxType, b: &BoxType, buf: &mut Vec<BoxType>) {
    if let Some(c) = get_overlap(a, b) {
        if a[0] < c[0] {
            buf.push([a[0], c[0] - 1, a[2], a[3], a[4], a[5]]);
        }
        if a[1] > c[1] {
            buf.push([c[1] + 1, a[1], a[2], a[3], a[4], a[5]]);
        }
        if a[2] < c[2] {
            buf.push([c[0], c[1], a[2], c[2] - 1, a[4], a[5]]);
        }
        if a[3] > c[3] {
            buf.push([c[0], c[1], c[3] + 1, a[3], a[4], a[5]]);
        }
        if a[4] < c[4] {
            buf.push([c[0], c[1], c[2], c[3], a[4], c[4] - 1]);
        }
        if a[5] > c[5] {
            buf.push([c[0], c[1], c[2], c[3], c[5] + 1, a[5]]);
        }
    } else {
        buf.push(*a);
    }
}

fn many_box_diff(
    boxes: &[BoxType],
    subbox: &BoxType,
    target: &mut Vec<BoxType>,
) {
    target.clear();
    for curbox in boxes {
        box_diff(curbox, subbox, target);
    }
}

fn volume(b: &BoxType) -> i64 {
    b.iter().tuples().map(|(a, b)| b - a + 1).product()
}

fn part2() {
    let mut screen = Vec::new();
    let mut screen_buf = Vec::new();

    let mut box_frags = Vec::new();
    let mut box_frags_buf = Vec::new();

    for (state, curbox) in
        BufReader::new(File::open("input/day22/input").unwrap())
            .lines()
            .map(|l| l.unwrap())
            .map(|l| {
                if let Some(c) = REG.captures(&l) {
                    (
                        match &c[1] {
                            "on" => true,
                            "off" => false,
                            _ => unreachable!(),
                        },
                        (2..=7)
                            .map(|i| c[i].parse().unwrap())
                            .collect::<ArrayVec<i64, 6>>()
                            .into_inner()
                            .unwrap(),
                    )
                } else {
                    unreachable!()
                }
            })
    {
        if state {
            box_frags.clear();
            box_frags.push(curbox);
            for cur_box in &screen {
                box_frags_buf.clear();
                many_box_diff(&box_frags, cur_box, &mut box_frags_buf);
                mem::swap(&mut box_frags, &mut box_frags_buf);
            }
            screen.extend(box_frags.iter());
        } else {
            screen_buf.clear();
            many_box_diff(&screen, &curbox, &mut screen_buf);
            mem::swap(&mut screen, &mut screen_buf);
        }
    }

    let ans: i64 = screen.iter().map(volume).sum();

    println!("{}", ans);
}
