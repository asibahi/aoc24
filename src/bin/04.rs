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

// bitset impl
pub fn part_one(input: &str) -> Option<u32> {
    use advent_of_code::bitset::Bitset;

    let mut xppp;
    let mut sppp;

    let mut xpp = Bitset::new();
    let mut mpp;
    let mut app;
    let mut spp = Bitset::new();

    let mut xp = Bitset::new();
    let mut mp = Bitset::new();
    let mut ap = Bitset::new();
    let mut sp = Bitset::new();

    let mut x = Bitset::new();
    let mut m = Bitset::new();
    let mut a = Bitset::new();
    let mut s = Bitset::new();

    let mut acc = 0;

    for line in input.lines() {
        xppp = xpp;
        xpp = xp;
        xp = x;
        x = Bitset::new();

        mpp = mp;
        mp = m;
        m = Bitset::new();

        app = ap;
        ap = a;
        a = Bitset::new();

        sppp = spp;
        spp = sp;
        sp = s;
        s = Bitset::new();

        for byte in line.as_bytes() {
            x = x.push_bit(*byte == b'X');
            m = m.push_bit(*byte == b'M');
            a = a.push_bit(*byte == b'A');
            s = s.push_bit(*byte == b'S');
        }

        acc += (x & m << 1 & a << 2 & s << 3).count_ones();
        acc += (x & m >> 1 & a >> 2 & s >> 3).count_ones();

        acc += (x & mp << 1 & app << 2 & sppp << 3).count_ones();
        acc += (x & mp >> 1 & app >> 2 & sppp >> 3).count_ones();
        acc += (x & mp & app & sppp).count_ones();

        acc += (xppp & mpp << 1 & ap << 2 & s << 3).count_ones();
        acc += (xppp & mpp >> 1 & ap >> 2 & s >> 3).count_ones();
        acc += (xppp & mpp & ap & s).count_ones();
    }

    Some(acc)
}

// much faster than the solution wiht nom ,sadly.
pub fn part_one_imp(input: &str) -> Option<u32> {
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
