use advent_of_code::BytesResult;
use nom::{
    bytes::complete::tag,
    character::complete::u64,
    combinator::map,
    sequence::{delimited, separated_pair},
};

advent_of_code::solution!(3);

fn parse_mul(input: &[u8]) -> BytesResult<u64> {
    map(
        delimited(tag("mul("), separated_pair(u64, tag(","), u64), tag(")")),
        |(a, b)| a * b,
    )(input)
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut ioo = input.as_bytes();
    let mut acc = 0;
    while !ioo.is_empty() {
        if ioo[0] != b'm' {
            ioo = &ioo[1..];
            continue;
        }

        if let Ok((rem, res)) = parse_mul(ioo) {
            ioo = rem;
            acc += res;
        } else {
            ioo = &ioo[1..];
        }
    }

    Some(acc)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut input = input.as_bytes();
    let mut acc = 0;

    while !input.is_empty() {
        while !input.is_empty() {
            if input[0] != b'd' && input[0] != b'm' {
                input = &input[1..];
                continue;
            }
            if let Ok((rem, result)) = parse_mul(input) {
                input = rem;
                acc += result
            } else if let Ok((rem, _)) = tag::<_, _, ()>("don't()")(input) {
                input = rem;
                break;
            } else {
                input = &input[1..]
            }
        }

        while !input.is_empty() {
            if input[0] != b'd' {
                input = &input[1..];
                continue;
            }
            if let Ok((rem, _)) = tag::<_, _, ()>("do()")(input) {
                input = rem;
                break;
            } else {
                input = &input[1..]
            }
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
