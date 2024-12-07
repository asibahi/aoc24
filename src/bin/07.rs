use advent_of_code::BytesResult;
use nom::{
    bytes::complete::{tag, take},
    character::complete::{newline, u64},
    multi::separated_list1,
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

fn validate_test(target: u64, list: &[u64], part2: bool) -> bool {
    if list.len() == 1 {
        return list[0] == target;
    }
    let mut add_list = vec![list[0] + list[1]];
    add_list.extend_from_slice(&list[2..]);

    if validate_test(target, &add_list, part2) {
        return true;
    }

    if part2 {
        let mut conc_list = vec![list[0] * 10u64.pow(list[1].ilog10() + 1) + list[1]];

        conc_list.extend_from_slice(&list[2..]);
        if validate_test(target, &conc_list, part2) {
            return true;
        };
    }

    let mut mul_list = vec![list[0] * list[1]];
    mul_list.extend_from_slice(&list[2..]);

    validate_test(target, &mul_list, part2)
}

pub fn part_one(input: &str) -> Option<u64> {
    let input = input.as_bytes();
    let (_, tests) = separated_list1(newline, parse_line)(input).ok()?;

    Some(
        tests
            .into_iter()
            .filter(|t| validate_test(t.target, &t.list, false))
            .map(|t| t.target)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let input = input.as_bytes();
    let (_, tests) = separated_list1(newline, parse_line)(input).ok()?;

    Some(
        tests
            .into_iter()
            .filter(|t| validate_test(t.target, &t.list, true))
            .map(|t| t.target)
            .sum(),
    )
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
        let r = validate_test(x.target, &x.list, false);
        let r2 = validate_test(x.target, &x.list, true);

        assert_eq!(r, y);
        assert_eq!(r2, z);
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
