use nom::{
    character::complete::{newline, space1, u64},
    combinator::map,
    multi::separated_list1,
    sequence::separated_pair,
};
use std::{collections::HashMap, ops::Add};

advent_of_code::solution!(1);

fn parse_line(input: &[u8]) -> nom::IResult<&[u8], (u64, u64)> {
    separated_pair(u64, space1, u64)(input)
}

fn parse_file(input: &[u8]) -> nom::IResult<&[u8], (Vec<u64>, Vec<u64>)> {
    map(separated_list1(newline, parse_line), |v| {
        v.into_iter().unzip()
    })(input)
}

pub fn part_one(input: &str) -> Option<u64> {
    let input = input.as_bytes();
    let (mut fst, mut snd) = parse_file(input).unwrap().1;

    fst.sort();
    snd.sort();

    fst.into_iter()
        .zip(snd)
        .map(|(a, b)| a.abs_diff(b))
        .reduce(Add::add)
}

pub fn part_two(input: &str) -> Option<u64> {
    let input = input.as_bytes();
    let (fst, snd) = parse_file(input).unwrap().1;

    let mut map = HashMap::new();
    for d in snd {
        map.entry(d).and_modify(|d| *d += 1).or_insert(1);
    }

    let mut sim = 0;
    for a in fst {
        sim += a * map.get(&a).copied().unwrap_or_default();
    }

    Some(sim)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
