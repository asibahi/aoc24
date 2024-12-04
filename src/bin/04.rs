use advent_of_code::BytesResult;
use nom::bytes::complete::{tag, take};

advent_of_code::solution!(4);

pub fn x(l: usize, input: &[u8]) -> BytesResult<()> {
    let (rem, _) = tag(b"X")(input)?;
    let (rem, _) = take(l)(rem)?;
    let (rem, _) = tag(b"M")(rem)?;
    let (rem, _) = take(l)(rem)?;
    let (rem, _) = tag(b"A")(rem)?;
    let (rem, _) = take(l)(rem)?;
    let (rem, _) = tag(b"S")(rem)?;

    Ok((rem, ()))
}
pub fn s(l: usize, input: &[u8]) -> BytesResult<()> {
    let (rem, _) = tag(b"S")(input)?;
    let (rem, _) = take(l)(rem)?;
    let (rem, _) = tag(b"A")(rem)?;
    let (rem, _) = take(l)(rem)?;
    let (rem, _) = tag(b"M")(rem)?;
    let (rem, _) = take(l)(rem)?;
    let (rem, _) = tag(b"X")(rem)?;

    Ok((rem, ()))
}

fn xmas(input: &[u8], ll: usize) -> BytesResult<'_, u32> {
    let mut acc = 0;
    for i in [0, ll, ll - 1, ll + 1] {
        if x(i, input).is_ok() {
            acc += 1;
        }
        if s(i, input).is_ok() {
            acc += 1
        }
    }
    Ok((&input[1..], acc))
}

pub fn part_one_nom(input: &str) -> Option<u32> {
    let line_len = input.lines().next().unwrap().len();
    let mut input = input.as_bytes();

    let mut acc = 0;

    while !input.is_empty() {
        if let Ok((_, res)) = xmas(input, line_len) {
            acc += res;
        }
        input = &input[1..]
    }

    Some(acc)
}

// much faster than the solution wiht nom ,sadly.
pub fn part_one(input: &str) -> Option<u32> {
    let line_len = input.lines().next().unwrap().len() + 1;
    let input = input.as_bytes();

    let mut acc = input.windows(4).fold(0, |acc, window| {
        if window == b"XMAS" || window == b"SAMX" {
            acc + 1u32
        } else {
            acc
        }
    });

    let diagonal = line_len - 1;
    for i in 0..input.len() - line_len * 3 {
        // if input[i] != b'X' && input[i] !=b'S' { continue;}
        let vert = [
            input[i],
            input[i + line_len],
            input[i + line_len * 2],
            input[i + line_len * 3],
        ];
        if &vert == b"XMAS" || &vert == b"SAMX" {
            acc += 1;
        }
        let diag = [
            input[i],
            input[i + diagonal],
            input[i + diagonal * 2],
            input[i + diagonal * 3],
        ];
        if &diag == b"XMAS" || &diag == b"SAMX" {
            acc += 1;
        }
    }

    let diagonal = line_len + 1;
    for i in 0..input.len() - line_len * 3 - 3 {
        let vert = [
            input[i],
            input[i + diagonal],
            input[i + diagonal * 2],
            input[i + diagonal * 3],
        ];
        if &vert == b"XMAS" || &vert == b"SAMX" {
            acc += 1;
        }
    }

    Some(acc)
}

pub fn part_two(input: &str) -> Option<u32> {
    let line_len = input.lines().next().unwrap().len() + 1;
    let input = input.as_bytes();

    let mut acc = 0;

    for i in line_len + 1..input.len() - line_len - 1 {
        if input[i] != b'A' {
            continue;
        }
        let dia = [input[i - line_len - 1], input[i], input[i + line_len + 1]];
        let di2 = [input[i - line_len + 1], input[i], input[i + line_len - 1]];

        if (&dia == b"MAS" || &dia == b"SAM") && (&di2 == b"MAS" || &di2 == b"SAM") {
            acc += 1;
        }
    }

    Some(acc)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
