

use advent_of_code::{Span, SpanResult};
use ahash::{HashMap, HashMapExt, HashSet, HashSetExt};
use itertools::Itertools;
use nom::character::complete::anychar
;

advent_of_code::solution!(8);

#[derive(Debug)]
struct Antenna {
    name: char,
    row: i32,
    col: i32,
}

fn parse_antenna(input: Span) -> SpanResult<Antenna> {
    let row = input.location_line() as i32 - 1;
    let col = input.get_column() as i32 - 1;

    let (input, name) = anychar(input)?;
    let antenna = Antenna { name, row, col };

    Ok((input, antenna))
}

pub fn part_one(input: &str) -> Option<u32> {
    let len = input.lines().count() as i32;
    let mut input = Span::new(input.as_bytes());
    let mut map: HashMap<char, Vec<(i32, i32)>> = HashMap::with_capacity(2000);
    let mut antinodes = HashSet::with_capacity(2000);

    while let Ok((rem, ant)) = parse_antenna(input) {
        input = rem;
        if ant.name != '.' && ant.name != '\n' {
            map.entry(ant.name).or_default().push((ant.row, ant.col));
        }
    }
    for (_, v) in map {
        let v = v.into_iter().tuple_combinations();

        for (a, b) in v {
            let c @ (cx, cy) = add(a,sub(a,b));
            if (0..len).contains(&cx) && (0..len).contains(&cy) {
                antinodes.insert(c);
            }
            let c @ (cx, cy) = add(b,sub(b,a));
            if (0..len).contains(&cx) && (0..len).contains(&cy) {
                antinodes.insert(c);
            }
        }
    }

    Some(antinodes.len() as u32)
}

fn add(lhs: (i32, i32), rhs:(i32,i32)) -> (i32, i32) {
    (lhs.0 + rhs.0, lhs.1 + rhs.1)
}
fn sub(lhs: (i32, i32), rhs:(i32,i32)) -> (i32, i32) {
    (lhs.0 - rhs.0, lhs.1 - rhs.1)
}

pub fn part_two(input: &str) -> Option<u32> {
    let len = input.lines().count() as i32;
    let mut input = Span::new(input.as_bytes());
    let mut map: HashMap<char, Vec<(i32, i32)>> = HashMap::with_capacity(2000);
    let mut antinodes = HashSet::with_capacity(2000);

    while let Ok((rem, ant)) = parse_antenna(input) {
        input = rem;
        if ant.name != '.' && ant.name != '\n' {
            map.entry(ant.name).or_default().push((ant.row, ant.col));

            antinodes.insert((ant.row, ant.col));
        }
    }

    for (_, v) in map {
        let v = v.into_iter().tuple_combinations();

        for (a, b) in v {

            let (mut cx, mut cy) = add(a, sub(a,b));
            while (0..len).contains(&cx) && (0..len).contains(&cy) {
                antinodes.insert((cx, cy));
                (cx , cy) = add((cx,cy), sub(a,b));
            }
            let (mut cx, mut cy) = add(b, sub(b,a));
            while (0..len).contains(&cx) && (0..len).contains(&cy) {
                antinodes.insert((cx, cy));
                (cx , cy) = add((cx,cy), sub(b,a));
            }
        }
    }

    Some(antinodes.len() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
