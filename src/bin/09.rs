// #![expect(unused)]
#![allow(clippy::needless_range_loop)]

use advent_of_code::BytesResult;
use nom::{bytes::complete::take, character::complete::u8, combinator::map_parser};

advent_of_code::solution!(9);

#[derive(Debug, Clone, Copy)]
enum Block {
    File { id: usize },
    Empty,
}

fn parse_block(input: &[u8]) -> BytesResult<u8> {
    map_parser(take(1u8), u8)(input)
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut input = input.as_bytes();

    let mut id = 0;
    let mut blocks = Vec::with_capacity(input.len() * 4);
    let mut file = false;

    while let Ok((rem, number)) = parse_block(input) {
        input = rem;
        file = !file;

        if file {
            for _ in 0..number {
                blocks.push(Block::File { id });
            }
            id += 1;
        } else {
            for _ in 0..number {
                blocks.push(Block::Empty);
            }
        }
    }

    for idx in 0..blocks.len() {
        let b = blocks[idx];
        if matches!(b, Block::File { .. }) {
            continue;
        }

        let mut rev_idx = blocks.len() - 1;
        loop {
            let b_r = blocks[rev_idx];
            if matches!(b_r, Block::File { .. }) {
                blocks[idx] = b_r;
                blocks[rev_idx] = Block::Empty;
                // rev_idx -= 1;
                break;
            }
            rev_idx -= 1;
        }
    }

    let res = blocks
        .into_iter()
        .filter(|b| matches!(b, Block::File { .. }))
        .enumerate()
        .map(|(idx, b)| {
            let Block::File { id } = b else {
                unreachable!()
            };

            idx * id
        })
        .sum();

    Some(res)
}

#[derive(Debug, Clone, Copy)]
struct CBlock {
    len: u8,
    block: Block,
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut input = input.as_bytes();

    let mut id = 0;
    let mut blocks = Vec::with_capacity(input.len());
    let mut file = false;

    while let Ok((rem, number)) = parse_block(input) {
        input = rem;
        file = !file;

        if file {
            blocks.push(CBlock {
                len: number,
                block: Block::File { id },
            });
            id += 1;
        } else {
            blocks.push(CBlock {
                len: number,
                block: Block::Empty,
            });
        }
    }

    let mut block_stack = blocks.clone();
    while let Some(last_block) = block_stack.pop() {
        let CBlock {
            len,
            block: Block::File { id },
        } = last_block
        else {
            continue;
        };

        let first_space = blocks
            .iter()
            .enumerate()
            .find(|(_, b)| matches!(b.block, Block::Empty if b.len >= len));
        let consumed_space = blocks
            .iter()
            .enumerate()
            .find(|(_, b)| matches!(b.block, Block::File { id: id_test } if id_test == id));

        let Some((empty_idx, _)) = first_space else {
            continue;
        };
        let Some((consumed_idx, _)) = consumed_space else {
            continue;
        };

        if consumed_idx <= empty_idx {
            continue;
        }
        blocks[consumed_idx] = CBlock {
            len,
            block: Block::Empty,
        };
        blocks[empty_idx] = CBlock {
            len: blocks[empty_idx].len - len,
            block: Block::Empty,
        };
        blocks.insert(empty_idx, last_block);
    }

    let res = blocks
        .into_iter()
        .flat_map(|CBlock { len, block }| [block].repeat(len as usize))
        .enumerate()
        .filter(|(_, b)| matches!(b, Block::File { .. }))
        .map(|(idx, b)| {
            let Block::File { id } = b else {
                unreachable!()
            };

            idx * id
        })
        .sum();

    Some(res)
}

fn _debug(blocks: &[CBlock]) {
    for CBlock { len, block } in blocks {
        for _ in 0..*len {
            match block {
                Block::File { id } => print!("{}", id % 10),
                Block::Empty => print!("-"),
            }
        }
    }
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}
