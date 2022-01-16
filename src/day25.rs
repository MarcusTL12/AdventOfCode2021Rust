use std::{fs, mem};

use itertools::Itertools;
use ndarray::{s, Array2};

pub const PARTS: [fn(); 2] = [part1, part2];

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
            b'.' => 0,
            b'>' => 1,
            b'v' => 2,
            b'\n' => 0,
            _ => panic!(),
        };
    }

    Array2::from_shape_vec((lines, linelen + 1), data)
        .unwrap()
        .slice(s![.., ..-1])
        .into_owned()
}

fn part1() {
    let mut m = parse_input("input/day25/input");
    let mut m_buf = m.clone();

    let (h, w) = m.shape().iter().cloned().collect_tuple().unwrap();

    let mut done = false;

    let mut i = 0;

    while !done {
        i += 1;
        done = true;
        for (a, b) in m.iter().zip(m_buf.iter_mut()) {
            *b = *a;
        }
        for ((i, j), &el) in m.indexed_iter() {
            if el == 1 && m[(i, (j + 1) % w)] == 0 {
                done = false;
                m_buf[(i, j)] = 0;
                m_buf[(i, (j + 1) % w)] = 1;
            }
        }

        mem::swap(&mut m, &mut m_buf);
        for (a, b) in m.iter().zip(m_buf.iter_mut()) {
            *b = *a;
        }

        for ((i, j), &el) in m.indexed_iter() {
            if el == 2 && m[((i + 1) % h, j)] == 0 {
                done = false;
                m_buf[(i, j)] = 0;
                m_buf[((i + 1) % h, j)] = 2;
            }
        }

        mem::swap(&mut m, &mut m_buf);
    }

    println!("{i}");
}

fn part2() {
    println!("Merry Christmas!");
}
