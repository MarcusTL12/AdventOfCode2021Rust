use std::fs;

use itertools::Itertools;
use ndarray::{s, Array2};

pub const PARTS: [fn(); 2] = [part1, part2];

const DIRS: [(isize, isize); 8] = [
    (0, 1),
    (1, 1),
    (1, 0),
    (1, -1),
    (0, -1),
    (-1, -1),
    (-1, 0),
    (-1, 1),
];

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
    let mut m = parse_input("input/day11/input");
    let mut m = m.slice_mut(s![.., ..-1]);

    let (h, w) = m.shape().iter().cloned().collect_tuple().unwrap();

    let mut total = 0;

    for _ in 0..100 {
        for x in m.iter_mut() {
            *x += 1;
        }

        loop {
            let mut didflash = false;

            for (i, j) in (0..h).cartesian_product(0..w) {
                if m[(i, j)] > 9 {
                    m[(i, j)] = 0;
                    didflash = true;
                    total += 1;
                    for (di, dj) in DIRS {
                        let ni = (i as isize + di) as usize;
                        let nj = (j as isize + dj) as usize;
                        match m.get_mut((ni, nj)) {
                            Some(x) if *x != 0 => *x += 1,
                            _ => (),
                        }
                    }
                }
            }

            if !didflash {
                break;
            }
        }
    }

    println!("{total}");
}

fn part2() {
    let mut m = parse_input("input/day11/input");
    let mut m = m.slice_mut(s![.., ..-1]);

    let (h, w) = m.shape().iter().cloned().collect_tuple().unwrap();

    for stepnum in 1.. {
        for x in m.iter_mut() {
            *x += 1;
        }

        loop {
            let mut didflash = false;

            for (i, j) in (0..h).cartesian_product(0..w) {
                if m[(i, j)] > 9 {
                    m[(i, j)] = 0;
                    didflash = true;
                    for (di, dj) in DIRS {
                        let ni = (i as isize + di) as usize;
                        let nj = (j as isize + dj) as usize;
                        match m.get_mut((ni, nj)) {
                            Some(x) if *x != 0 => *x += 1,
                            _ => (),
                        }
                    }
                }
            }

            if !didflash {
                break;
            }
        }

        if m.iter().all(|&x| x == 0) {
            println!("{stepnum}");
            break;
        }
    }
}
