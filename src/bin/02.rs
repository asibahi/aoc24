use advent_of_code::BytesResult;
use itertools::Itertools;
use nom::{
    character::complete::{newline, space1, u8},
    combinator::iterator,
    multi::separated_list1,
    sequence::terminated,
};
use std::cmp::Ordering;

advent_of_code::solution!(2);

fn parse_line(input: &[u8]) -> BytesResult<Vec<u8>> {
    separated_list1(space1, u8)(input)
}

fn is_safe_1(list: impl Iterator<Item = u8>) -> bool {
    let list = list.tuple_windows();
    let mut ordering = Ordering::Equal;
    for (a, b) in list {
        let c = a.cmp(&b);
        ordering = ordering.then(c);
        if !(c == ordering && (1..=3).contains(&a.abs_diff(b))) {
            return false;
        }
    }
    true
}

fn is_safe_2(list: &[u8]) -> bool {
    is_safe_1(list.iter().copied())
        || (0..list.len()).any(|doomed_idx| {
            let trimmed_list = list[..doomed_idx]
                .iter()
                .chain(&list[doomed_idx + 1..])
                .copied();

            is_safe_1(trimmed_list)
        })
}

pub fn part_one(input: &str) -> Option<u32> {
    let input = input.as_bytes();
    let res = iterator(input, terminated(parse_line, newline))
        .filter(|v| is_safe_1(v.iter().copied()))
        .count();

    Some(res as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let input = input.as_bytes();
    let res = iterator(input, terminated(parse_line, newline))
        .filter(|v| is_safe_2(v))
        .count();

    Some(res as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
