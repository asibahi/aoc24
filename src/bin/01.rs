use nom::{
    character::complete::{newline, space1, u64},
    combinator::map,
    multi::separated_list1,
    sequence::separated_pair,
};
advent_of_code::solution!(1);

fn parse_line(input: &str) -> nom::IResult<&str, (u64, u64)> {
    separated_pair(u64, space1, u64)(input)
}

fn parse_file(input: &str) -> nom::IResult<&str, (Vec<u64>, Vec<u64>)> {
    map(separated_list1(newline, parse_line), |v| {
        v.into_iter().unzip()
    })(input)
}

pub fn part_one(input: &str) -> Option<u64> {
    let (mut fst, mut snd) = parse_file(input).unwrap().1;

    fst.sort();
    snd.sort();

    let mut acc = 0;

    for (a, b) in fst.into_iter().zip(snd) {
        acc += a.abs_diff(b);
    }

    Some(acc)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (fst, snd) = parse_file(input).unwrap().1;

    let mut sim = 0;
    for a in fst {
        let count = snd.iter().filter(|b| a == **b).count();
        sim += a * count as u64;
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
