use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub const PARTS: [fn(); 2] = [part1, part2];

fn parse_input(filename: &str) -> [u64; 9] {
    BufReader::new(File::open(filename).unwrap())
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .split(',')
        .map(|x| x.parse().unwrap())
        .fold([0; 9], |mut fish, x: usize| {
            fish[x] += 1;
            fish
        })
}

fn part1() {
    let mut fish = parse_input("input/day6/input");

    for _ in 0..80 {
        let new_fish = fish[0];
        for i in 1..=8 {
            fish[i - 1] = fish[i];
        }
        fish[6] += new_fish;
        fish[8] = new_fish;
    }

    let ans: u64 = fish.iter().sum();

    println!("{}", ans);
}

fn part2() {
    let mut fish = parse_input("input/day6/input");

    for _ in 0..256 {
        let new_fish = fish[0];
        for i in 1..=8 {
            fish[i - 1] = fish[i];
        }
        fish[6] += new_fish;
        fish[8] = new_fish;
    }

    let ans: u64 = fish.iter().sum();

    println!("{}", ans);
}
