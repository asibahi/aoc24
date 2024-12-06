// #![expect(unused)]

// use advent_of_code::BytesResult;
use ahash::{HashMap, HashMapExt, HashSet, HashSetExt};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

advent_of_code::solution!(6);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Right,
    Left,
    Down,
}

#[derive(Debug)]
enum Element {
    Obstacle,
    Unvisited,
    Visited,
}

#[derive(Debug, Clone)]
enum Element2 {
    Obstacle,
    HotPath,
    Unvisited,
    Visited(Vec<Direction>),
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
        let (x, y) = guard_loc;
        let forward = match dir {
            Direction::Up => (x - 1, y),
            Direction::Right => (x, y + 1),
            Direction::Left => (x, y - 1),
            Direction::Down => (x + 1, y),
        };

        match map.get(&forward) {
            Some(Element::Obstacle) => {
                dir = match dir {
                    Direction::Up => Direction::Right,
                    Direction::Right => Direction::Down,
                    Direction::Left => Direction::Up,
                    Direction::Down => Direction::Left,
                };
            }
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
    let mut guard_loc = (0, 0);

    for (idx, line) in input.lines().enumerate() {
        let idx = idx + 1;
        for (jdx, c) in line.char_indices() {
            let jdx = jdx + 1;
            match c {
                '#' => map.insert((idx, jdx), Element2::Obstacle),
                '.' => map.insert((idx, jdx), Element2::Unvisited),
                '^' => {
                    guard_loc = (idx, jdx);
                    map.insert((idx, jdx), Element2::Visited(vec![Direction::Up]))
                }
                _ => unreachable!(),
            };
        }
    }

    let mut set = HashSet::with_capacity(6000);
    {
        let mut guard_loc = guard_loc;
        let mut dir = Direction::Up;
        loop {
            let (x, y) = guard_loc;
            let forward = match dir {
                Direction::Up => (x - 1, y),
                Direction::Right => (x, y + 1),
                Direction::Left => (x, y - 1),
                Direction::Down => (x + 1, y),
            };

            match map.get(&forward) {
                Some(Element2::Obstacle) => {
                    dir = match dir {
                        Direction::Up => Direction::Right,
                        Direction::Right => Direction::Down,
                        Direction::Left => Direction::Up,
                        Direction::Down => Direction::Left,
                    };
                }
                Some(Element2::Unvisited) => {
                    guard_loc = forward;
                    set.insert(forward);
                    map.insert(forward, Element2::HotPath);
                }
                None => break,
                _ => guard_loc = forward,
            }
        }
    }

    let counter = set
        .par_iter()
        .filter(|(idx, jdx)| {
            let mut map = map.clone();

            map.insert((*idx, *jdx), Element2::Obstacle);

            // ==
            let mut dir = Direction::Up;
            let mut guard_loc = guard_loc;

            loop {
                let (x, y) = guard_loc;
                let forward = match dir {
                    Direction::Up => (x - 1, y),
                    Direction::Right => (x, y + 1),
                    Direction::Left => (x, y - 1),
                    Direction::Down => (x + 1, y),
                };

                match map.get_mut(&forward) {
                    Some(Element2::Obstacle) => {
                        dir = match dir {
                            Direction::Up => Direction::Right,
                            Direction::Right => Direction::Down,
                            Direction::Left => Direction::Up,
                            Direction::Down => Direction::Left,
                        };
                    }
                    Some(Element2::Unvisited | Element2::HotPath) => {
                        guard_loc = forward;
                        map.insert(forward, Element2::Visited(vec![dir]));
                    }
                    Some(Element2::Visited(ref mut v)) => {
                        if v.contains(&dir) {
                            return true;
                        }
                        v.push(dir);
                        guard_loc = forward
                    }
                    None => return false,
                }
            }
            //==
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
