use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use itertools::Itertools;
use scan_fmt::scan_fmt;

pub const PARTS: [fn(); 2] = [part1, part2];

fn part1() {
    let l = BufReader::new(File::open("input/day17/input").unwrap())
        .lines()
        .next()
        .unwrap()
        .unwrap();

    let (minx, maxx, miny, maxy) =
        scan_fmt!(&l, "target area: x={}..{}, y={}..{}", i64, i64, i64, i64)
            .unwrap();

    let ans = (1..=maxx)
        .cartesian_product(1..=200)
        .filter_map(|(mut vx, mut vy)| {
            let mut x = 0;
            let mut y = 0;
            let mut highest = 0;
            let mut hit = false;
            while x <= maxx && y >= maxy {
                x += vx;
                y += vy;
                vx -= vx.signum();
                vy -= 1;
                highest = if y > highest { y } else { highest };
                if minx <= x && x <= maxx && miny <= y && y <= maxy {
                    hit = true;
                    break;
                }
            }
            if hit {
                Some(highest)
            } else {
                None
            }
        })
        .max()
        .unwrap();

    println!("{}", ans);
}

fn part2() {
    let l = BufReader::new(File::open("input/day17/input").unwrap())
        .lines()
        .next()
        .unwrap()
        .unwrap();

    let (minx, maxx, miny, maxy) =
        scan_fmt!(&l, "target area: x={}..{}, y={}..{}", i64, i64, i64, i64)
            .unwrap();

    let ans = (1..=maxx)
        .cartesian_product(miny..=200)
        .filter(|(mut vx, mut vy)| {
            let mut x = 0;
            let mut y = 0;
            let mut hit = false;
            while x <= maxx && y >= miny {
                x += vx;
                y += vy;
                vx -= vx.signum();
                vy -= 1;
                if minx <= x && x <= maxx && miny <= y && y <= maxy {
                    hit = true;
                    break;
                }
            }
            hit
        })
        .count();

    println!("{}", ans);
}
