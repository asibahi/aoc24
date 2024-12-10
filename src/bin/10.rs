use advent_of_code::{Span, SpanResult};
use ahash::{HashMap, HashMapExt, HashSet, HashSetExt};
use nom::{
    bytes::complete::{tag, take},
    character::complete::u8,
    combinator::{map_parser, opt},
};

advent_of_code::solution!(10);

type Position = (u32, u32);

fn parse_pos(input: Span) -> SpanResult<(u8, Position)> {
    let (input, _) = opt(tag("\n"))(input)?;
    let row = input.location_line();
    let col = input.get_column() as u32;
    let (input, height) = map_parser(take(1u8), u8)(input)?;

    Ok((input, (height, (row, col))))
}

fn hike_score(
    pos: Position,
    target: u8,
    map: &HashMap<Position, u8>,
    set: &mut HashSet<Position>,
) {
    match map.get(&pos) {
        None => return,
        Some(&v) if v != target => return,
        Some(&9) => {
            set.insert(pos);
            return;
        }
        Some(_) => ()
    }

    hike_score((pos.0 + 1, pos.1), target + 1, map, set);
    hike_score((pos.0 - 1, pos.1), target + 1, map, set);
    hike_score((pos.0, pos.1 + 1), target + 1, map, set);
    hike_score((pos.0, pos.1 - 1), target + 1, map, set);
}

fn hike_rating(
    pos: Position,
    target: u8,
    map: &HashMap<Position, u8>,
) -> usize {
    match map.get(&pos) {
        None => return 0,
        Some(&v) if v != target => return 0,
        Some(&9) => return 1,
        Some(_) => ()
    }

    let mut acc = 0;

    acc += hike_rating((pos.0 + 1, pos.1), target + 1, map);
    acc += hike_rating((pos.0 - 1, pos.1), target + 1, map);
    acc += hike_rating((pos.0, pos.1 + 1), target + 1, map);
    acc += hike_rating((pos.0, pos.1 - 1), target + 1, map);

    acc
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut map = HashMap::with_capacity(input.len());
    let mut input = Span::new(input.as_bytes());

    let mut heads = Vec::new();

    while let Ok((rem, (h, pos))) = parse_pos(input) {
        input = rem;
        map.insert(pos, h);
        if h == 0 {
            heads.push(pos);
        }
    }

    let mut acc = 0;

    for pos in heads {
        let mut set = HashSet::new();
        hike_score(pos, 0, &map, &mut set);

        acc += set.len();
    }

    Some(acc)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut map = HashMap::with_capacity(input.len());
    let mut input = Span::new(input.as_bytes());

    let mut heads = Vec::new();

    while let Ok((rem, (h, pos))) = parse_pos(input) {
        input = rem;
        map.insert(pos, h);
        if h == 0 {
            heads.push(pos);
        }
    }

    let mut acc = 0;

    for pos in heads {
        acc += hike_rating(pos, 0, &map);
    }

    Some(acc)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
