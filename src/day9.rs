use std::fs;

use ndarray::{s, Array2, ArrayViewMut2};

pub const PARTS: [fn(); 2] = [part1, part2];

const DIRS: [(isize, isize); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

fn parse_input(filename: &str) -> Array2<u8> {
    let mut data = fs::read(filename).unwrap();

    let linelen = data
        .iter()
        .enumerate()
        .find_map(|(i, &c)| if c == b'\n' { Some(i) } else { None })
        .unwrap();

    let lines = data.len() / (linelen + 1);

    for c in data.iter_mut() {
        *c = match *c {
            b'0'..=b'9' => *c - b'0',
            b'\n' => 0,
            _ => panic!(),
        };
    }

    Array2::from_shape_vec((lines, linelen + 1), data).unwrap()
}

fn part1() {
    let m = parse_input("input/day9/input");
    let m = m.slice(s![.., ..-1]);

    let ans: u32 = m
        .indexed_iter()
        .filter_map(|((i, j), &x)| {
            if DIRS
                .iter()
                .map(|(di, dj)| {
                    [(i as isize + di) as usize, (j as isize + dj) as usize]
                })
                .filter_map(|p| m.get(p))
                .all(|&y| y > x)
            {
                Some(x as u32 + 1)
            } else {
                None
            }
        })
        .sum();
    
    println!("{}", ans);
}

fn traverse_from(pos: [usize; 2], m: &mut ArrayViewMut2<u8>) -> usize {
    let mut s = 0;

    m[pos] = 9;

    for (di, dj) in DIRS {
        let ni = (pos[0] as isize + di) as usize;
        let nj = (pos[1] as isize + dj) as usize;
        let np = [ni, nj];
        if matches!(m.get(np), Some(9)) {
            s += traverse_from(np, m);
        }
    }

    s
}

fn part2() {
    let mut m = parse_input("input/day9/input");
    let m = m.slice_mut(s![.., ..-1]);

    todo!()
}
