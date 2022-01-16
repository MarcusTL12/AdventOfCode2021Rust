use std::{collections::HashSet, fs::read_to_string, mem::take, thread, vec};

use arrayvec::ArrayVec;
use crossbeam_channel::unbounded;
use itertools::Itertools;

pub const PARTS: [fn(); 2] = [part1, part2];

type Point = [i64; 3];

fn parse_input(filename: &str) -> Vec<Vec<Point>> {
    read_to_string(filename)
        .unwrap()
        .split("\n\n")
        .map(|s| {
            s.split('\n')
                .skip(1)
                .map(|l| {
                    l.split(',')
                        .map(|n| n.parse().unwrap())
                        .collect::<ArrayVec<_, 3>>()
                        .into_inner()
                        .unwrap()
                })
                .collect()
        })
        .collect()
}

fn rotate(p: &Point, i: usize) -> Point {
    match i {
        0 => *p,
        1 => [p[0], -p[2], p[1]],
        2 => [p[2], p[1], -p[0]],
        3 => [-p[2], p[1], p[0]],
        4 => [p[2], -p[1], p[0]],
        _ => unimplemented!(),
    }
}

const ROT_GEN: [usize; 24] = [
    0, 1, 1, 1, 2, 1, 1, 1, 2, 1, 1, 1, 3, 1, 1, 1, 3, 1, 1, 1, 3, 1, 1, 1,
];

fn rotate_scanner(scanner: &mut [Point], i: usize) {
    for p in scanner.iter_mut() {
        *p = rotate(p, i);
    }
}

fn add_pts(a: &Point, b: &Point) -> Point {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
}

fn sub_pts(a: &Point, b: &Point) -> Point {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

fn offset_scanner(scanner: &mut [Point], offset: &Point) {
    for p in scanner.iter_mut() {
        *p = add_pts(p, offset);
    }
}

fn find_overlap(s1: &[Point], s2: &mut [Point]) -> Option<Point> {
    for rot in ROT_GEN {
        rotate_scanner(s2, rot);
        for (i, j) in (0..s1.len()).cartesian_product(0..s2.len()) {
            let s2_pos = sub_pts(&s1[i], &s2[j]);
            if s1
                .iter()
                .filter(|k| {
                    sub_pts(k, &s2_pos).iter().map(|x| x.abs()).max().unwrap()
                        <= 1000
                })
                .count()
                == 12
            {
                let mut amt_overlap = 0;
                for (k, l) in (0..s1.len()).cartesian_product(0..s2.len()) {
                    amt_overlap += (s1[k] == add_pts(&s2[l], &s2_pos)) as usize;
                    if amt_overlap >= 12 {
                        offset_scanner(s2, &s2_pos);
                        return Some(s2_pos);
                    }
                }
            }
        }
    }
    rotate_scanner(s2, 4);
    None
}

// Single threaded solution
fn _part1() {
    let mut inp = parse_input("input/day19/input");

    let mut fixed = HashSet::new();
    fixed.insert(0);

    while fixed.len() < inp.len() {
        for i in 0..inp.len() {
            if fixed.contains(&i) {
                for j in 0..inp.len() {
                    if i != j && !fixed.contains(&j) {
                        let mut tmp = take(&mut inp[j]);
                        if find_overlap(&inp[i], &mut tmp).is_some() {
                            fixed.insert(j);
                        }
                        inp[j] = tmp;
                    }
                }
            }
        }
    }

    let ans = inp
        .into_iter()
        .flat_map(|s| s.into_iter())
        .collect::<HashSet<_>>()
        .len();

    println!("{ans}");
}

fn part1() {
    let mut inp = parse_input("input/day19/input");

    let mut fixed = HashSet::new();
    fixed.insert(0);

    while fixed.len() < inp.len() {
        for i in 0..inp.len() {
            if fixed.contains(&i) {
                let (distributer_s, distributer_r) =
                    unbounded::<(usize, Vec<_>, Vec<_>)>();
                let (collector_s, collector_r) = unbounded();

                let handles: Vec<_> = (0..num_cpus::get())
                    .map(|_| {
                        let collector_s = collector_s.clone();
                        let distributer_r = distributer_r.clone();
                        thread::spawn(move || {
                            for (j, a, mut b) in distributer_r.iter() {
                                if find_overlap(&a, &mut b).is_some() {
                                    collector_s.send((j, b, true)).unwrap();
                                } else {
                                    collector_s.send((j, b, false)).unwrap();
                                }
                            }
                        })
                    })
                    .collect();

                let mut amt_sent = 0;
                for j in 0..inp.len() {
                    if i != j && !fixed.contains(&j) {
                        let tmp = take(&mut inp[j]);
                        distributer_s.send((j, inp[i].clone(), tmp)).unwrap();
                        amt_sent += 1;
                    }
                }
                for _ in 0..amt_sent {
                    let (j, tmp, b) = collector_r.recv().unwrap();
                    if b {
                        fixed.insert(j);
                    }
                    inp[j] = tmp;
                }

                drop(distributer_s);

                for handle in handles {
                    handle.join().unwrap();
                }
            }
        }
    }

    let ans = inp
        .into_iter()
        .flat_map(|s| s.into_iter())
        .collect::<HashSet<_>>()
        .len();

    println!("{ans}");
}

// Single threaded solution
fn _part2() {
    let mut inp = parse_input("input/day19/input");

    let mut fixed = HashSet::new();
    fixed.insert(0);

    let mut scanners = vec![[0, 0, 0]];

    while fixed.len() < inp.len() {
        for i in 0..inp.len() {
            if fixed.contains(&i) {
                for j in 0..inp.len() {
                    if i != j && !fixed.contains(&j) {
                        let mut tmp = take(&mut inp[j]);
                        if let Some(p) = find_overlap(&inp[i], &mut tmp) {
                            fixed.insert(j);
                            scanners.push(p);
                        }
                        inp[j] = tmp;
                    }
                }
            }
        }
    }

    let ans = scanners
        .iter()
        .cartesian_product(scanners.iter())
        .map(|(a, b)| {
            a.iter()
                .zip(b.iter())
                .map(|(a, b)| (a - b).abs())
                .sum::<i64>()
        })
        .max()
        .unwrap();

    println!("{ans}");
}

fn part2() {
    let mut inp = parse_input("input/day19/input");

    let mut fixed = HashSet::new();
    fixed.insert(0);

    let mut scanners = vec![[0, 0, 0]];

    while fixed.len() < inp.len() {
        for i in 0..inp.len() {
            if fixed.contains(&i) {
                let (distributer_s, distributer_r) =
                    unbounded::<(usize, Vec<_>, Vec<_>)>();
                let (collector_s, collector_r) = unbounded();

                let handles: Vec<_> = (0..num_cpus::get())
                    .map(|_| {
                        let collector_s = collector_s.clone();
                        let distributer_r = distributer_r.clone();
                        thread::spawn(move || {
                            for (j, a, mut b) in distributer_r.iter() {
                                let p = find_overlap(&a, &mut b);
                                collector_s.send((j, b, p)).unwrap();
                            }
                        })
                    })
                    .collect();

                let mut amt_sent = 0;
                for j in 0..inp.len() {
                    if i != j && !fixed.contains(&j) {
                        let tmp = take(&mut inp[j]);
                        distributer_s.send((j, inp[i].clone(), tmp)).unwrap();
                        amt_sent += 1;
                    }
                }
                for _ in 0..amt_sent {
                    let (j, tmp, b) = collector_r.recv().unwrap();
                    if let Some(p) = b {
                        fixed.insert(j);
                        scanners.push(p);
                    }
                    inp[j] = tmp;
                }

                drop(distributer_s);

                for handle in handles {
                    handle.join().unwrap();
                }
            }
        }
    }

    let ans = scanners
        .iter()
        .cartesian_product(scanners.iter())
        .map(|(a, b)| {
            a.iter()
                .zip(b.iter())
                .map(|(a, b)| (a - b).abs())
                .sum::<i64>()
        })
        .max()
        .unwrap();

    println!("{ans}");
}
