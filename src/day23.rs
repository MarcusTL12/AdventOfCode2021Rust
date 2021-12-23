use std::{cmp::Reverse, fs::read_to_string};

use hashbrown::HashSet;
use itertools::Itertools;
use priority_queue::PriorityQueue;

pub const PARTS: [fn(); 2] = [part1, part2];

fn parse_input(filename: &str) -> ([u8; 4], [u8; 4]) {
    read_to_string(filename)
        .unwrap()
        .chars()
        .filter(|c| c.is_alphabetic())
        .map(|c| c as u8 - b'A' + 1)
        .tuples()
        .map(|(a, b, c, d)| [a, b, c, d])
        .collect_tuple()
        .unwrap()
}

fn _print_board<const N: usize>(floors: &[[u8; 4]; N], hallway: &[u8; 11]) {
    println!("#############");
    print!("#");
    for &i in hallway {
        print!(
            "{}",
            match i {
                0 => '.',
                _ => (b'A' + i - 1) as char,
            }
        )
    }
    println!("#");
    print!("###");
    let mut floors = floors.iter();
    for i in floors.next().unwrap() {
        print!(
            "{}#",
            match i {
                0 => '.',
                _ => (b'A' + i - 1) as char,
            }
        )
    }
    println!("#");
    for floor in floors {
        print!("  #");
        for i in floor {
            print!(
                "{}#",
                match i {
                    0 => '.',
                    _ => (b'A' + i - 1) as char,
                }
            )
        }
        println!();
    }
    println!("  #########  ");
}

fn solve<const N: usize>(floors: [[u8; 4]; N]) -> usize {
    fn pow10(n: usize) -> usize {
        (0..n).fold(1, |x, _| x * 10)
    }

    fn room_inds(n: usize) -> usize {
        (n + 1) * 2
    }

    fn abs_diff(a: usize, b: usize) -> usize {
        match a.cmp(&b) {
            std::cmp::Ordering::Equal => 0,
            std::cmp::Ordering::Greater => a - b,
            std::cmp::Ordering::Less => b - a,
        }
    }

    fn move_out<I: Iterator<Item = usize>, const N: usize>(
        it: I,
        k: &mut ([[u8; 4]; N], [u8; 11]),
        depth: usize,
        i: usize,
        energy: usize,
        visited: &HashSet<([[u8; 4]; N], [u8; 11])>,
        queue: &mut PriorityQueue<([[u8; 4]; N], [u8; 11]), Reverse<usize>>,
    ) {
        for j in it {
            if k.1[j] != 0 {
                break;
            } else if !matches!(j, 2 | 4 | 6 | 8) {
                k.1[j] = k.0[depth][i];
                k.0[depth][i] = 0;

                let new_energy = energy
                    + (depth + 1 + abs_diff(room_inds(i), j))
                        * pow10((k.1[j] - 1) as usize);

                if !visited.contains(k) {
                    queue.push_increase(*k, Reverse(new_energy));
                }

                k.0[depth][i] = k.1[j];
                k.1[j] = 0;
            }
        }
    }

    let hallway = [0; 11];

    let mut queue = PriorityQueue::new();
    queue.push((floors, hallway), Reverse(0));

    let mut visited = HashSet::new();
    visited.insert((floors, hallway));

    while let Some((mut k, energy)) = queue.pop() {
        visited.insert(k);

        if k.0.iter().all(|&f| f == [1, 2, 3, 4]) && k.1.iter().all(|&x| x == 0)
        {
            return energy.0;
        }

        for i in 0..4 {
            if let Some(depth) = (0..N).find(|&d| k.0[d][i] != 0) {
                if (depth..N).any(|d| k.0[d][i] != (i + 1) as u8) {
                    move_out(
                        (0..room_inds(i)).rev(),
                        &mut k,
                        depth,
                        i,
                        energy.0,
                        &visited,
                        &mut queue,
                    );
                    move_out(
                        room_inds(i) + 1..11,
                        &mut k,
                        depth,
                        i,
                        energy.0,
                        &visited,
                        &mut queue,
                    );
                }
            }
        }

        for i in 0..11 {
            if k.1[i] != 0 {
                let pod_ind = k.1[i] - 1;
                let room_ind = room_inds(pod_ind as usize);
                k.1[i] = 0;

                if (i.min(room_ind)..=i.max(room_ind)).all(|j| k.1[j] == 0)
                    && k.0
                        .iter()
                        .map(|f| f[pod_ind as usize])
                        .all(|x| x == 0 || x == pod_ind + 1)
                {
                    if let Some(depth) =
                        (0..N).rev().find(|&d| k.0[d][pod_ind as usize] == 0)
                    {
                        let new_energy = energy.0
                            + (depth + 1 + abs_diff(room_ind, i))
                                * pow10(pod_ind as usize);

                        k.0[depth][pod_ind as usize] = pod_ind + 1;

                        if !visited.contains(&k) {
                            queue.push_increase(k, Reverse(new_energy));
                        }

                        k.0[depth][pod_ind as usize] = 0;
                    }
                }

                k.1[i] = pod_ind + 1;
            }
        }
    }

    panic!("Did not find solution!")
}

fn part1() {
    let (top, bottom) = parse_input("input/day23/input");

    let ans = solve([top, bottom]);

    println!("{}", ans);
}

fn part2() {
    let (top, bottom) = parse_input("input/day23/input");

    let ans = solve([top, [4, 3, 2, 1], [4, 2, 1, 3], bottom]);

    println!("{}", ans);
}
