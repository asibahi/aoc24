use advent_of_code::BytesResult;
use nom::{
    bytes::complete::{tag, take},
    character::complete::{newline, u64},
    combinator::iterator,
    multi::separated_list1,
    sequence::terminated,
};

advent_of_code::solution!(7);

struct Test {
    target: u64,
    list: Vec<u64>,
}

fn parse_line(input: &[u8]) -> BytesResult<Test> {
    let (input, target) = u64(input)?;
    let (input, _) = take(2u8)(input)?;
    let (input, list) = separated_list1(tag(" "), u64)(input)?;

    Ok((input, Test { target, list }))
}

fn conc(lhs: u64, rhs: u64) -> u64 {
    lhs * 10u64.pow(rhs.ilog10() + 1) + rhs
}

fn validate_test(target: u64, list: &[u64], acc: u64, part2: bool) -> bool {
    if list.is_empty() {
        return acc == target;
    }

    acc <= target
        && (validate_test(target, &list[1..], acc + list[0], part2)
            || validate_test(target, &list[1..], acc * list[0], part2)
            || (part2 && validate_test(target, &list[1..], conc(acc, list[0]), part2)))
}

pub fn part_one(input: &str) -> Option<u64> {
    let res = iterator(input.as_bytes(), terminated(parse_line, newline))
        .filter(|t| validate_test(t.target, &t.list[1..], t.list[0], false))
        .map(|t| t.target)
        .sum();

    Some(res)
}

pub fn part_two(input: &str) -> Option<u64> {
    use rayon::iter::{ParallelBridge, ParallelIterator};
    let res = iterator(input.as_bytes(), terminated(parse_line, newline))
        .par_bridge()
        .filter(|t| validate_test(t.target, &t.list[1..], t.list[0], true))
        .map(|t| t.target)
        .sum();

    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(Test{target: 190, list: vec![10, 19]}, (true, true))]
    #[test_case(Test{target: 3267, list: vec![81, 40, 27]}, (true, true))]
    #[test_case(Test{target: 83, list: vec![17, 5]}, (false, false))]
    #[test_case(Test{target: 156, list: vec![15, 6]}, (false, true))]
    #[test_case(Test{target: 7290, list: vec![6, 8, 6, 15]}, (false, true))]
    #[test_case(Test{target: 161011, list: vec![16, 10, 13]}, (false, false))]
    #[test_case(Test{target: 192, list: vec![17, 8, 14]}, (false, true))]
    #[test_case(Test{target: 21037, list: vec![9, 7, 18, 13]}, (false, false))]
    #[test_case(Test{target: 292, list: vec![11, 6, 16, 20]}, (true, true))]
    fn test_validate(x: Test, (y, z): (bool, bool)) {
        let r = validate_test(x.target, &x.list[1..], x.list[0], false);
        let r2 = validate_test(x.target, &x.list[1..], x.list[0], true);

        assert_eq!(r, y);
        assert_eq!(r2, z);
    }

    #[test_case((15,6), 156)]
    #[test_case((15,600), 15600)]
    fn test_conc(x: (u64, u64), y: u64) {
        let r = conc(x.0, x.1);
        assert_eq!(r, y)
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
