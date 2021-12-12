use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

use itertools::Itertools;

pub const PARTS: [fn(); 2] = [part1, part2];

fn parse_input(filename: &str) -> (Vec<Vec<usize>>, Vec<bool>) {
    let mut node_translation: HashMap<_, _> = ["start", "end"]
        .iter()
        .enumerate()
        .map(|(i, &s)| (s.to_owned(), i))
        .collect();

    let mut get_node_id = |s: &str| {
        if let Some(&x) = node_translation.get(s) {
            x
        } else {
            let l = node_translation.len();
            node_translation.insert(s.to_owned(), l);
            l
        }
    };

    let mut graph = Vec::new();

    let mut add_edge = |a, b| {
        while graph.len() <= a {
            graph.push(Vec::new())
        }
        graph[a].push(b);
    };

    let mut small_caves = Vec::new();

    let mut add_small_cave = |a: &str, ai: usize| {
        while small_caves.len() <= ai {
            small_caves.push(false);
        }
        if a.chars().all(|c| c.is_ascii_lowercase()) {
            small_caves[ai] = true;
        }
    };

    for l in BufReader::new(File::open(filename).unwrap())
        .lines()
        .map(|l| l.unwrap())
    {
        let (a, b) = l.split('-').collect_tuple().unwrap();

        let ai = get_node_id(a);
        let bi = get_node_id(b);

        add_edge(ai, bi);
        add_edge(bi, ai);

        add_small_cave(a, ai);
        add_small_cave(b, bi);
    }

    small_caves[0] = false;
    small_caves[1] = false;

    (graph, small_caves)
}

fn remaining_paths_from1(
    graph: &[Vec<usize>],
    curpos: usize,
    small_caves: &[bool],
    remaining_nodes: &mut [bool],
) -> usize {
    let mut amt = 0;

    for &node in &graph[curpos] {
        if node == 1 {
            amt += 1;
        } else if remaining_nodes[node] {
            if small_caves[node] {
                remaining_nodes[node] = false;
            }
            amt += remaining_paths_from1(
                graph,
                node,
                small_caves,
                remaining_nodes,
            );
            remaining_nodes[node] = true;
        }
    }

    amt
}

fn part1() {
    let (graph, small_caves) = parse_input("input/day12/input");

    let mut remaining = vec![true; graph.len()];
    remaining[0] = false;

    let ans = remaining_paths_from1(&graph, 0, &small_caves, &mut remaining);

    println!("{}", ans);
}

fn remaining_paths_from2(
    graph: &[Vec<usize>],
    curpos: usize,
    small_caves: &[bool],
    remaining_nodes: &mut [bool],
    spent_double: bool,
) -> usize {
    let mut amt = 0;

    for &node in &graph[curpos] {
        if node == 1 {
            amt += 1;
        } else if node != 0 && (remaining_nodes[node] || !spent_double) {
            let mut removed = false;
            let mut n_spent_double = spent_double;
            if small_caves[node] {
                if remaining_nodes[node] {
                    removed = true;
                    remaining_nodes[node] = false;
                } else {
                    n_spent_double = true;
                }
            }
            amt += remaining_paths_from2(
                graph,
                node,
                small_caves,
                remaining_nodes,
                n_spent_double
            );
            if removed {
                remaining_nodes[node] = true;
            }
        }
    }

    amt
}

fn part2() {
    let (graph, small_caves) = parse_input("input/day12/input");

    let mut remaining = vec![true; graph.len()];
    remaining[0] = false;

    let ans =
        remaining_paths_from2(&graph, 0, &small_caves, &mut remaining, false);

    println!("{}", ans);
}
