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

        for _ in 0..number {
            if file {
                blocks.push(Block::File { id });
            } else {
                blocks.push(Block::Empty);
            }
        }
        id += file as usize;
    }

    let mut empty_idx = 0;
    while let Some(last_block) = blocks.pop() {
        if matches!(last_block, Block::Empty) {
            continue;
        }
        let Some((idx, _)) = blocks
            .iter()
            .enumerate()
            .skip(empty_idx)
            .find(|(_, b)| matches!(b, Block::Empty))
        else {
            blocks.push(last_block);
            break;
        };
        empty_idx = idx;
        blocks[idx] = last_block;
    }

    let res = blocks
        .into_iter()
        .enumerate()
        .map(|(idx, b)| {
            let Block::File { id } = b else {
                unreachable!();
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
    let mut blocks = Vec::with_capacity(input.len() * 2);
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

        let mut biter = blocks.iter().enumerate();
        let Some((e_idx, e_len)) = biter
            .find(|(_, b)| matches!(b.block, Block::Empty if b.len >= len))
            .map(|(i, b)| (i, b.len))
        else {
            continue;
        };
        let Some(c_idx) = biter
            .find(|(_, b)| matches!(b.block, Block::File { id: id_test } if id_test == id))
            .map(|(i, _)| i)
        else {
            continue;
        };

        blocks[c_idx] = CBlock {
            len,
            block: Block::Empty,
        };

        if e_len == len {
            blocks[e_idx] = last_block;
        } else {
            blocks[e_idx].len -= len;
            blocks.insert(e_idx, last_block);
        }
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
