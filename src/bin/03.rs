use advent_of_code::BytesResult;
use itertools::Either::{self, *};
use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    character::complete::u64,
    combinator::{map, value},
    multi::many1,
    sequence::{delimited, separated_pair},
};

advent_of_code::solution!(3);

fn parse_mul(input: &[u8]) -> BytesResult<u64> {
    map(
        delimited(tag("mul("), separated_pair(u64, tag(","), u64), tag(")")),
        |(a, b)| a * b,
    )(input)
}

fn parse_one(input: &[u8]) -> BytesResult<u64> {
    map(many1(alt((parse_mul, value(0, take(1u8))))), |v| {
        v.into_iter().sum()
    })(input)
}

fn parse_two(input: &[u8]) -> BytesResult<Either<u64, bool>> {
    alt((
        map(parse_mul, Left),
        value(Right(true), tag("do()")),
        value(Right(false), tag("don't()")),
        value(Left(0), take(1u8)),
    ))(input)
}

pub fn part_one(input: &str) -> Option<u64> {
    let ret = parse_one(input.as_bytes()).unwrap().1;
    Some(ret)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut input = input.as_bytes();

    let mut acc = 0;
    let mut do_ = true;

    while let Ok((rem, result)) = parse_two(input) {
        input = rem;

        match result {
            Left(v) if do_ => acc += v,
            Right(cond) => do_ = cond,
            _ => (),
        }
    }

    Some(acc)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(48));
    }
}
