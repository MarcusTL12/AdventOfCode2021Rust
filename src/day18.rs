use std::{
    fs::File,
    io::{BufRead, BufReader},
    iter::{Peekable, Sum},
    ops::Add,
};

use itertools::Itertools;

pub const PARTS: [fn(); 2] = [part1, part2];

#[derive(Clone, Debug)]
enum Node {
    Leaf(u64),
    Branch(Box<Node>, Box<Node>),
}

impl From<&str> for Node {
    fn from(s: &str) -> Node {
        fn parse_node<I: Iterator<Item = char>>(
            chars: &mut Peekable<I>,
        ) -> Node {
            let c = chars.next().unwrap();
            match c {
                '[' => {
                    let left = parse_node(chars);
                    chars.next();
                    while let Some(']' | ',') = chars.peek() {
                        chars.next();
                    }
                    let right = parse_node(chars);
                    Node::Branch(Box::from(left), Box::from(right))
                }
                '0'..='9' => {
                    let mut n = c as u64 - b'0' as u64;
                    while let Some('0'..='9') = chars.peek() {
                        let c = chars.next().unwrap();
                        n = 10 * n + c as u64 - b'0' as u64;
                    }
                    Node::Leaf(n)
                }
                _ => unreachable!(),
            }
        }

        parse_node(&mut s.chars().peekable())
    }
}

impl Node {
    fn reduce(mut self) -> Self {
        fn explode(
            node: &mut Node,
            depth: usize,
        ) -> Option<(Option<u64>, Option<u64>)> {
            fn add_first_right(node: &mut Node, val: u64) {
                match node {
                    Node::Leaf(x) => *x += val,
                    Node::Branch(l, _) => add_first_right(l, val),
                }
            }

            fn add_first_left(node: &mut Node, val: u64) {
                match node {
                    Node::Leaf(x) => *x += val,
                    Node::Branch(_, r) => add_first_left(r, val),
                }
            }

            match node {
                Node::Branch(l, r) => match (&**l, &**r) {
                    (&Node::Leaf(l), &Node::Leaf(r)) if depth >= 4 => {
                        *node = Node::Leaf(0);
                        Some((Some(l), Some(r)))
                    }
                    _ => match explode(l, depth + 1) {
                        None => match explode(r, depth + 1) {
                            Some((a, b)) => {
                                if let Some(a) = a {
                                    add_first_left(l, a);
                                }
                                Some((None, b))
                            }
                            x => x,
                        },
                        Some((a, b)) => {
                            if let Some(b) = b {
                                add_first_right(r, b);
                            }
                            Some((a, None))
                        }
                    },
                },
                Node::Leaf(_) => None,
            }
        }

        fn split(node: &mut Node) -> bool {
            match node {
                &mut Node::Leaf(x) if x >= 10 => {
                    let a = x / 2;
                    let b = a + x % 2;
                    *node = Node::Branch(
                        Box::from(Node::Leaf(a)),
                        Box::from(Node::Leaf(b)),
                    );
                    true
                }
                Node::Branch(l, r) => {
                    if split(l) {
                        true
                    } else {
                        split(r)
                    }
                }
                _ => false,
            }
        }

        while {
            while let Some(_) = explode(&mut self, 0) {}
            split(&mut self)
        } {}

        self
    }

    fn magnitude(&self) -> u64 {
        match self {
            &Node::Leaf(x) => x,
            Node::Branch(l, r) => l.magnitude() * 3 + r.magnitude() * 2,
        }
    }
}

impl Add<Node> for Node {
    type Output = Node;

    fn add(self, rhs: Node) -> Self {
        Node::Branch(Box::from(self), Box::from(rhs)).reduce()
    }
}

impl Sum for Node {
    fn sum<I: Iterator<Item = Node>>(mut iter: I) -> Self {
        let mut acc = iter.next().unwrap();
        for n in iter {
            acc = acc + n;
        }
        acc
    }
}

fn part1() {
    let ans = BufReader::new(File::open("input/day18/input").unwrap())
        .lines()
        .map(|l| l.unwrap())
        .map(|l| Node::from(l.as_str()))
        .sum::<Node>()
        .magnitude();

    println!("{}", ans);
}

fn part2() {
    let snails: Vec<_> =
        BufReader::new(File::open("input/day18/input").unwrap())
            .lines()
            .map(|l| l.unwrap())
            .map(|l| Node::from(l.as_str()))
            .collect();

    let ans = (0..snails.len())
        .cartesian_product(0..snails.len())
        .filter(|(i, j)| i != j)
        .map(|(i, j)| (snails[i].clone() + snails[j].clone()).magnitude())
        .max()
        .unwrap();

    println!("{}", ans);
}
