use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub const PARTS: [fn(); 2] = [part1, part2];

fn collect_binary<I: Iterator<Item = bool>>(biterator: I) -> u64 {
    biterator.fold(0, |x, b| (x << 1) | if b { 1 } else { 0 })
}

fn do_ins<I: Iterator<Item = u64>>(mut it: I, t: u64) -> u64 {
    match t {
        0 => it.sum(),
        1 => it.product(),
        2 => it.min().unwrap(),
        3 => it.max().unwrap(),
        5 => (it.next().unwrap() > it.next().unwrap()) as u64,
        6 => (it.next().unwrap() < it.next().unwrap()) as u64,
        7 => (it.next().unwrap() == it.next().unwrap()) as u64,
        _ => unreachable!(),
    }
}

fn parse_packet<I: Iterator<Item = bool>>(
    biterator: &mut I,
) -> (u64, usize, u64) {
    let mut v = collect_binary(biterator.take(3));

    let t = collect_binary(biterator.take(3));

    let mut bits = 6;

    if t == 4 {
        let mut n = 0;
        while let Some(true) = biterator.next() {
            bits += 5;
            n = (n << 4) | collect_binary(biterator.take(4));
        }
        bits += 5;
        ((n << 4) | collect_binary(biterator.take(4)), bits, v)
    } else {
        bits += 1;
        if let Some(false) = biterator.next() {
            let nbits = collect_binary(biterator.take(15)) as usize;
            bits += 15;
            let it = (0..).scan(0, |b, _| {
                if *b < nbits {
                    let (n, db, dv) = parse_packet(biterator);
                    *b += db;
                    bits += db;
                    v += dv;
                    Some(n)
                } else {
                    None
                }
            });

            (do_ins(it, t), bits, v)
        } else {
            let npackets = collect_binary(biterator.take(11)) as usize;
            bits += 11;
            let it = (0..).scan(0, |np, _| {
                if *np < npackets {
                    let (n, db, dv) = parse_packet(biterator);
                    *np += 1;
                    bits += db;
                    v += dv;
                    Some(n)
                } else {
                    None
                }
            });

            (do_ins(it, t), bits, v)
        }
    }
}

fn get_biterator<'a>(l: &'a str) -> impl Iterator<Item = bool> + 'a {
    l.chars().flat_map(|c| {
        match c {
            '0' => [false, false, false, false],
            '1' => [false, false, false, true],
            '2' => [false, false, true, false],
            '3' => [false, false, true, true],
            '4' => [false, true, false, false],
            '5' => [false, true, false, true],
            '6' => [false, true, true, false],
            '7' => [false, true, true, true],
            '8' => [true, false, false, false],
            '9' => [true, false, false, true],
            'A' => [true, false, true, false],
            'B' => [true, false, true, true],
            'C' => [true, true, false, false],
            'D' => [true, true, false, true],
            'E' => [true, true, true, false],
            'F' => [true, true, true, true],
            _ => unreachable!(),
        }
        .into_iter()
    })
}

fn part1() {
    let l = BufReader::new(File::open("input/day16/input").unwrap())
        .lines()
        .next()
        .unwrap()
        .unwrap();

    let mut biterator = get_biterator(&l);

    let (_, _, ans) = parse_packet(&mut biterator);

    println!("{ans}");
}

fn part2() {
    let l = BufReader::new(File::open("input/day16/input").unwrap())
        .lines()
        .next()
        .unwrap()
        .unwrap();

    let mut biterator = get_biterator(&l);

    let (ans, _, _) = parse_packet(&mut biterator);

    println!("{ans}");
}
