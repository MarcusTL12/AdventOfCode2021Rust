use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use stats::median;

pub const PARTS: [fn(); 2] = [part1, part2];

fn part1() {
    let mut stack = Vec::new();

    let ans: u64 = BufReader::new(File::open("input/day10/input").unwrap())
        .lines()
        .map(|l| l.unwrap())
        .filter_map(|l| {
            stack.clear();
            let mut s = None;
            for c in l.chars() {
                match c {
                    '(' | '[' | '{' | '<' => stack.push(c),
                    _ => {
                        if c != match stack.pop() {
                            Some('(') => ')',
                            Some('[') => ']',
                            Some('{') => '}',
                            Some('<') => '>',
                            _ => unreachable!(),
                        } {
                            s = Some(match c {
                                ')' => 3,
                                ']' => 57,
                                '}' => 1197,
                                '>' => 25137,
                                _ => unreachable!(),
                            });
                            break;
                        }
                    }
                }
            }
            s
        })
        .sum();

    println!("{}", ans);
}

fn part2() {
    let mut stack = Vec::new();

    let ans = median(
        BufReader::new(File::open("input/day10/input").unwrap())
            .lines()
            .map(|l| l.unwrap())
            .filter_map(|l| {
                stack.clear();
                let mut corrupted = false;
                for c in l.chars() {
                    match c {
                        '(' | '[' | '{' | '<' => stack.push(c),
                        _ => {
                            if c != match stack.pop() {
                                Some('(') => ')',
                                Some('[') => ']',
                                Some('{') => '}',
                                Some('<') => '>',
                                _ => unreachable!(),
                            } {
                                corrupted = true;
                                break;
                            }
                        }
                    }
                }

                if !corrupted && !stack.is_empty() {
                    let mut score = 0u64;
                    while let Some(c) = stack.pop() {
                        score = score * 5
                            + match c {
                                '(' => 1,
                                '[' => 2,
                                '{' => 3,
                                '<' => 4,
                                _ => unreachable!(),
                            };
                    }
                    Some(score)
                } else {
                    None
                }
            }),
    )
    .unwrap() as u64;

    println!("{}", ans);
}
