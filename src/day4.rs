use std::fs::read_to_string;

use ndarray::Array2;

pub const PARTS: [fn(); 2] = [part1, part2];

fn parse_input(filename: &str) -> (Vec<u32>, Vec<Array2<Option<u32>>>) {
    let tmp = read_to_string(filename).unwrap();

    let mut blocks = tmp.split("\n\n");

    let nums: Vec<u32> = blocks
        .next()
        .unwrap()
        .split(",")
        .map(|n| n.parse().unwrap())
        .collect();

    let boards = blocks
        .map(|b| {
            Array2::from_shape_vec(
                (5, 5),
                b.split_ascii_whitespace()
                    .map(|n| Some(n.parse().unwrap()))
                    .collect(),
            )
            .unwrap()
        })
        .collect();

    (nums, boards)
}

fn get_score(board: &Array2<Option<u32>>) -> Option<u32> {
    let tmp = board
        .rows()
        .into_iter()
        .any(|r| r.iter().all(|c| c.is_none()))
        || board
            .columns()
            .into_iter()
            .any(|r| r.iter().all(|c| c.is_none()));

    if tmp {
        Some(board.iter().filter_map(|&c| c).sum())
    } else {
        None
    }
}

fn play_board(
    nums: &[u32],
    board: &mut Array2<Option<u32>>,
) -> (usize, u32) {
    for (i, &n) in nums.iter().enumerate() {
        for c in board.iter_mut() {
            match *c {
                Some(m) if m == n => *c = None,
                _ => {}
            }
        }

        if let Some(score) = get_score(board) {
            return (i, score * n);
        }
    }

    unreachable!()
}

fn part1() {
    let (nums, mut boards) = parse_input("input/day4/input");

    let (_, score) = boards
        .iter_mut()
        .map(|board| play_board(&nums, board))
        .min_by_key(|&(a, _)| a)
        .unwrap();

    println!("{score}");
}

fn part2() {
    let (nums, mut boards) = parse_input("input/day4/input");

    let (_, score) = boards
        .iter_mut()
        .map(|board| play_board(&nums, board))
        .max_by_key(|&(a, _)| a)
        .unwrap();

    println!("{score}");
}
