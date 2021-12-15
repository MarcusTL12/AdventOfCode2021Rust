use std::{cmp::Reverse, collections::HashSet, fs};

use ndarray::{s, Array2};
use priority_queue::PriorityQueue;

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
    let m = parse_input("input/day15/input");
    let m = m.slice(s![.., ..-1]);

    let mut queue = PriorityQueue::new();
    queue.push((0, 0), Reverse(0));

    let mut visited = HashSet::new();
    visited.insert((0, 0));

    let mut ans = 0;

    while let Some(((i, j), l)) = queue.pop() {
        visited.insert((i, j));

        if [i + 1, j + 1] == m.shape() {
            ans = l.0;
            break;
        }

        for (di, dj) in &DIRS {
            let ni = (i as isize + di) as usize;
            let nj = (j as isize + dj) as usize;
            if let Some(&nl) = m.get((ni, nj)) {
                if !visited.contains(&(ni, nj)) {
                    queue.push_increase((ni, nj), Reverse(l.0 + nl as usize));
                }
            }
        }
    }

    println!("{}", ans);
}

fn part2() {
    let m = parse_input("input/day15/input");
    let m = {
        let m = m.slice(s![.., ..-1]);

        let (h, w) = {
            let s = m.shape();
            (s[0], s[1])
        };

        let mut mm = Array2::zeros((5 * h, 5 * w));

        for i in 0..5 * h {
            for j in 0..5 * w {
                let sub_i = i % h;
                let sub_j = j % w;
                let adder = i / h + j / h;

                mm[(i, j)] =
                    (((m[(sub_i, sub_j)] as usize + adder) - 1) % 9 + 1) as u8;
            }
        }

        mm
    };

    let mut queue = PriorityQueue::new();
    queue.push((0, 0), Reverse(0));

    let mut visited = HashSet::new();
    visited.insert((0, 0));

    let mut ans = 0;

    while let Some(((i, j), l)) = queue.pop() {
        visited.insert((i, j));

        if [i + 1, j + 1] == m.shape() {
            ans = l.0;
            break;
        }

        for (di, dj) in &DIRS {
            let ni = (i as isize + di) as usize;
            let nj = (j as isize + dj) as usize;
            if let Some(&nl) = m.get((ni, nj)) {
                if !visited.contains(&(ni, nj)) {
                    queue.push_increase((ni, nj), Reverse(l.0 + nl as usize));
                }
            }
        }
    }

    println!("{}", ans);
}
