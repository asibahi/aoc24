// use advent_of_code::BytesResult;

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let line_len = input.lines().next().unwrap().len() + 1;
    let input = input.as_bytes();

    let mut _hor = input.windows(4).fold(0, |acc, window| {
        if window == b"XMAS" || window == b"SAMX" {
            acc + 1u32
        } else {
            acc
        }
    });

    for i in 0..input.len() - line_len * 3 {
        let vert = [
            input[i],
            input[i + line_len],
            input[i + line_len * 2],
            input[i + line_len * 3],
        ];
        if &vert == b"XMAS" || &vert == b"SAMX" {
            _hor += 1;
        }
    }

    let diagonal = line_len - 1;
    for i in 0..input.len() - line_len * 3 {
        let vert = [
            input[i],
            input[i + diagonal],
            input[i + diagonal * 2],
            input[i + diagonal * 3],
        ];
        if &vert == b"XMAS" || &vert == b"SAMX" {
            _hor += 1;
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
            _hor += 1;
        }
    }

    Some(_hor)
}

pub fn part_two(input: &str) -> Option<u32> {
    let line_len = input.lines().next().unwrap().len() + 1;
    let input = input.as_bytes();

    let mut acc = 0;

    for i in line_len + 1..input.len() - line_len - 1 {
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
