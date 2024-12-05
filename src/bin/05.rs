use std::cmp::Ordering;

use advent_of_code::BytesResult;
use nom::{
    bytes::complete::{tag, take},
    character::complete::{newline, u32},
    combinator::verify,
    multi::separated_list0,
    sequence::separated_pair,
};

advent_of_code::solution!(5);

fn parse_rules(input: &[u8]) -> BytesResult<Vec<(u32, u32)>> {
    separated_list0(newline, separated_pair(u32, tag("|"), u32))(input)
}

fn parse_manuals(input: &[u8]) -> BytesResult<Vec<Vec<u32>>> {
    separated_list0(
        newline,
        verify(separated_list0(tag(","), u32), |v: &[u32]| !v.is_empty()),
    )(input)
}

fn validate_manual(manual: &[u32], rules: &[(u32, u32)]) -> bool {
    manual.is_sorted_by(|a, b| rules.contains(&(*a, *b)))
}

pub fn part_one(input: &str) -> Option<u32> {
    let input = input.as_bytes();

    let (input, rules) = parse_rules(input).ok()?;
    let (input, _) = take::<_, _, ()>(2u8)(input).ok()?;
    let (_, manuals) = parse_manuals(input).ok()?;

    let mut acc = 0;
    for manual in manuals {
        if !validate_manual(&manual, &rules) {
            continue;
        }

        let mid = manual[(manual.len() - 1) / 2];
        acc += mid;
    }

    Some(acc)
}

fn fix_manual(manual: &mut [u32], rules: &[(u32, u32)]) {
    manual.sort_by(|a, b| {
        if rules.contains(&(*a, *b)) {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    });
}

pub fn part_two(input: &str) -> Option<u32> {
    let input = input.as_bytes();

    let (input, rules) = parse_rules(input).ok()?;
    let (input, _) = take::<_, _, ()>(2u8)(input).ok()?;
    let (_, manuals) = parse_manuals(input).ok()?;

    // dbg!(&manuals);
    let mut acc = 0;
    // dumb solution
    for mut manual in manuals {
        if validate_manual(&manual, &rules) {
            continue;
        }

        fix_manual(&mut manual, &rules);

        let mid = manual[(manual.len() - 1) / 2];
        acc += mid;
    }

    Some(acc)
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec!(75,47,61,53,29), true)]
    #[test_case(vec!(97,61,53,29,13), true)]
    #[test_case(vec!(75,29,13), true)]
    #[test_case(vec!(75,97,47,61,53), false)]
    #[test_case(vec!(61,13,29), false)]
    #[test_case(vec!(97,13,75,29,47), false)]
    fn test_validation(x: Vec<u32>, y: bool) {
        let example = advent_of_code::template::read_file("examples", DAY);
        let rules = parse_rules(example.as_bytes()).unwrap().1;

        let res = validate_manual(&x, &rules);

        assert_eq!(res, y);
    }

    #[test_case(vec!  [75,97,47,61,53], vec! [97,75,47,61,53])]
    #[test_case(vec!  [61,13,29], vec! [61,29,13])]
    #[test_case(vec!  [97,13,75,29,47], vec! [97,75,47,29,13])]
    fn test_fix(mut x: Vec<u32>, y: Vec<u32>) {
        let example = advent_of_code::template::read_file("examples", DAY);
        let rules = parse_rules(example.as_bytes()).unwrap().1;

        fix_manual(&mut x, &rules);

        assert_eq!(x, y);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
