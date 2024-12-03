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

pub fn part_one_runtime(input: &str) -> Option<u64> {
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
pub fn part_one(_: &str) -> Option<u64> {
    const INPUT:&[u8] = include_bytes!("../../data/inputs/03.txt");
    const ANSWER : u64 = const {
        let mut ioo = INPUT;
        let mut acc = 0;

        while !ioo.is_empty() {
            if ioo[0] != b'm' {
                ioo = unsafe {
                    let l = ioo.len();
                    let p  = ioo.as_ptr();

                    core::slice::from_raw_parts(p.wrapping_add(1), l - 1)
                };
                continue;
            }
    
            if let Some((rem, res)) = 'v: {
                let mut length = 8;
                let lhs;
                let rhs;

                let mut cursor;

                if ioo.len() < length { break 'v None }
                if ioo[1] == b'u' && ioo[2] == b'l' && ioo[3] == b'(' {
                    let d4 = ioo[4];
                    let d5 = ioo[5];
                    let d6 = ioo[6];
                    if d4.is_ascii_digit() && d5.is_ascii_digit() && d6.is_ascii_digit()  {
                        lhs = 100 * (d4 - b'0') as u64 + 10 * (d5 - b'0') as u64 + (d6 - b'0') as u64;
                        cursor = 7;
                        length += 2;
                    } else if d4.is_ascii_digit() && d5.is_ascii_digit() {
                        lhs = 10 * (d4 - b'0') as u64 + (d5 - b'0') as u64;
                        cursor = 6;
                        length += 1;
                    } else if d4.is_ascii_digit() {
                        lhs = (d4 - b'0') as u64;
                        cursor = 5;
                    } else {
                        break 'v None;
                    }
                    if ioo[cursor] != b',' || ioo.len() < length { break 'v None }
                    let d4 = ioo[cursor + 1];
                    let d5 = ioo[cursor + 2];
                    let d6 = ioo[cursor + 3];
                    if d4.is_ascii_digit() && d5.is_ascii_digit() && d6.is_ascii_digit()  {
                        rhs = 100 * (d4 - b'0') as u64 + 10 * (d5 - b'0') as u64 + (d6 - b'0') as u64;
                        cursor += 4;
                        length += 2;
                    } else if d4.is_ascii_digit() && d5.is_ascii_digit() {
                        rhs = 10 * (d4 - b'0') as u64 + (d5 - b'0') as u64;
                        cursor += 3;
                        length += 1;
                    } else if d4.is_ascii_digit() {
                        rhs = (d4 - b'0') as u64;
                        cursor += 2;
                    } else {
                        break 'v None;
                    }
                    if ioo[cursor] != b')' || ioo.len() < length { break 'v None }


                } else {
                    break 'v None
                }

                let rem = unsafe {
                    let l = ioo.len();
                    let p  = ioo.as_ptr();

                    core::slice::from_raw_parts(p.wrapping_add(length), l - length)
                };
                Some((rem, lhs * rhs))
             }{
                ioo = rem;
                acc += res;
            } else {
                ioo = unsafe {
                    let l = ioo.len();
                    let p  = ioo.as_ptr();

                    core::slice::from_raw_parts(p.wrapping_add(1), l - 1)
                };
            }
        }

        acc
    };
   

    Some(ANSWER)
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
