// #![expect(unused)]

// use advent_of_code::BytesResult;
use ahash::{HashMap, HashMapExt, HashSet, HashSetExt};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

advent_of_code::solution!(6);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
enum Direction {
    Up = 1,
    Right = 2,
    Left = 4,
    Down = 8,
}
impl Direction {
    fn forward(self, (x, y): (usize, usize)) -> (usize, usize) {
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

#[derive(Debug)]
enum Element {
    Obstacle,
    Unvisited,
    Visited,
}

#[derive(Debug, Clone, Copy)]
enum Element2 {
    Obstacle,
    HotPath,
    Unvisited,
    Visited(u8),
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut map = HashMap::new();
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
                    map.insert((idx, jdx), Element::Visited)
                }
                _ => unreachable!(),
            };
        }
    }

    let mut counter = 1;
    let mut dir = Direction::Up;

    loop {
        let forward = dir.forward(guard_loc);

        match map.get(&forward) {
            Some(Element::Obstacle) => dir = dir.rotate(),
            Some(Element::Unvisited) => {
                counter += 1;
                guard_loc = forward;
                map.insert(forward, Element::Visited);
            }
            Some(Element::Visited) => guard_loc = forward,
            None => break,
        }
    }

    Some(counter)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut map = HashMap::new();
    let mut start = (0, 0);

    for (idx, line) in input.lines().enumerate() {
        let idx = idx + 1;
        for (jdx, c) in line.char_indices() {
            let jdx = jdx + 1;
            match c {
                '#' => map.insert((idx, jdx), Element2::Obstacle),
                '.' => map.insert((idx, jdx), Element2::Unvisited),
                '^' => {
                    start = (idx, jdx);
                    map.insert((idx, jdx), Element2::Visited(Direction::Up as u8))
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
            Some(Element2::Obstacle) => dir = dir.rotate(),
            Some(Element2::Unvisited) => {
                guard_loc = forward;
                set.insert(forward);
                map.insert(forward, Element2::HotPath);
            }
            _ => guard_loc = forward,
        }
    }

    let counter = set
        .par_iter()
        .filter(|(idx, jdx)| {
            let mut map = map.clone();

            map.insert((*idx, *jdx), Element2::Obstacle);

            let mut dir = Direction::Up;
            let mut guard_loc = start;

            loop {
                let forward = dir.forward(guard_loc);

                match map.get_mut(&forward) {
                    Some(Element2::Visited(v)) if *v & dir as u8 > 0 => return true,
                    None => return false,

                    Some(Element2::Obstacle) => dir = dir.rotate(),
                    Some(Element2::Unvisited | Element2::HotPath) => {
                        guard_loc = forward;
                        map.insert(forward, Element2::Visited(dir as u8));
                    }
                    Some(Element2::Visited(ref mut v)) => {
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
