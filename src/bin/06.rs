// #![expect(unused)]

// use advent_of_code::BytesResult;
use advent_of_code::{Span, SpanResult};
use ahash::{HashMap, HashMapExt, HashSet, HashSetExt};
use nom::{
    character::complete::{newline, one_of},
    combinator::{iterator, opt},
    sequence::preceded,
};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

advent_of_code::solution!(6);

type Position = (usize, usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
enum Direction {
    Up = 1,
    Right = 2,
    Left = 4,
    Down = 8,
}
impl Direction {
    fn forward(self, (x, y): Position) -> Position {
        match self {
            Direction::Up => (x - 1, y),
            Direction::Right => (x, y + 1),
            Direction::Left => (x, y - 1),
            Direction::Down => (x + 1, y),
        }
    }
    fn rotate(self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Left => Self::Up,
            Self::Down => Self::Left,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Element {
    Obstacle,
    Unvisited,
    Visited(u8),
}

fn parse_token(input: Span) -> SpanResult<(Position, Element)> {
    let row = input.location_line() as usize;
    let col = input.get_column();

    let (input, e) = one_of("#.^")(input)?;
    let e = match e {
        '#' => Element::Obstacle,
        '.' => Element::Unvisited,
        '^' => Element::Visited(0),
        _ => unreachable!(),
    };

    Ok((input, ((row, col), e)))
}

fn parse_grid(input: Span) -> SpanResult<(HashMap<Position, Element>, Position)> {
    let mut v = iterator(input, preceded(opt(newline), parse_token));
    let mut start = (0, 0);
    let map = HashMap::from_iter(v.inspect(|(k, v)| {
        if matches!(v, Element::Visited(_)) {
            start = *k;
        }
    }));
    let (input, _) = v.finish()?;

    Ok((input, (map, start)))
}

pub fn part_one_nom(input: &str) -> Option<u32> {
    let input = Span::new(input.as_bytes());

    let (_, (map, start)) = parse_grid(input).ok()?;

    let counter = walk(start, map);

    Some(counter)
}

fn walk(mut guard_loc: Position, mut map: HashMap<Position, Element>) -> u32 {
    let mut dir = Direction::Up;
    let mut counter = 1;
    loop {
        let forward = dir.forward(guard_loc);

        match map.get(&forward) {
            Some(Element::Obstacle) => dir = dir.rotate(),
            Some(Element::Unvisited) => {
                counter += 1;
                guard_loc = forward;
                map.insert(forward, Element::Visited(0));
            }
            Some(Element::Visited(_)) => guard_loc = forward,
            None => break,
        }
    }
    counter
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut map = HashMap::with_capacity(7000);
    let mut guard_loc = (0, 0);

    for (idx, line) in input.lines().enumerate() {
        let idx = idx + 1;
        for (jdx, c) in line.char_indices() {
            let jdx = jdx + 1;
            match c {
                '#' => map.insert((idx, jdx), Element::Obstacle),
                '.' => map.insert((idx, jdx), Element::Unvisited),
                '^' => {
                    guard_loc = (idx, jdx);
                    map.insert((idx, jdx), Element::Visited(0))
                }
                _ => unreachable!(),
            };
        }
    }

    let counter = walk(guard_loc, map);

    Some(counter)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut map = HashMap::with_capacity(7000);
    let mut start = (0, 0);

    for (idx, line) in input.lines().enumerate() {
        let idx = idx + 1;
        for (jdx, c) in line.char_indices() {
            let jdx = jdx + 1;
            match c {
                '#' => map.insert((idx, jdx), Element::Obstacle),
                '.' => map.insert((idx, jdx), Element::Unvisited),
                '^' => {
                    start = (idx, jdx);
                    map.insert((idx, jdx), Element::Visited(Direction::Up as u8))
                }
                _ => unreachable!(),
            };
        }
    }

    let mut set = HashSet::with_capacity(6000);
    let mut guard_loc = start;
    let mut dir = Direction::Up;
    loop {
        let forward = dir.forward(guard_loc);
        match map.get(&forward) {
            None => break,
            Some(Element::Obstacle) => dir = dir.rotate(),
            Some(Element::Unvisited) => {
                guard_loc = forward;
                set.insert(forward);
            }
            _ => guard_loc = forward,
        }
    }

    let counter = set
        .par_iter()
        .filter(|(idx, jdx)| {
            let mut map = map.clone();

            map.insert((*idx, *jdx), Element::Obstacle);

            let mut dir = Direction::Up;
            let mut guard_loc = start;

            loop {
                let forward = dir.forward(guard_loc);

                match map.get_mut(&forward) {
                    Some(Element::Visited(v)) if *v & dir as u8 > 0 => return true,
                    None => return false,

                    Some(Element::Obstacle) => dir = dir.rotate(),
                    Some(Element::Unvisited) => {
                        guard_loc = forward;
                        *map.get_mut(&forward).unwrap() = Element::Visited(dir as u8);
                    }
                    Some(Element::Visited(ref mut v)) => {
                        *v |= dir as u8;
                        guard_loc = forward
                    }
                }
            }
        })
        .count();

    Some(counter as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
