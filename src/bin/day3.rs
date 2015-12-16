use std::slice::Iter;
use std::io::{self, Read};

#[derive(Debug, PartialEq)]
enum Direction {
    North,
    South,
    East,
    West,
}

type Location = (i32, i32);

struct HousePath<'a> {
    location: Option<Location>,
    directions: Iter<'a, Direction>,
}

impl<'a> Iterator for HousePath<'a> {
    type Item = Location;

    fn next(&mut self) -> Option<Location> {
        // fetch current location
        // short-circuit if we're already done
        let location = match self.location {
            Some(location) => location,
            None => { return None; },
        };

        // calculate next location
        self.location = match self.directions.next() {
            Some(&Direction::North) =>
                Some((location.0, location.1 + 1)),
            Some(&Direction::South) =>
                Some((location.0, location.1 - 1)),
            Some(&Direction::East) =>
                Some((location.0 + 1, location.1)),
            Some(&Direction::West) =>
                Some((location.0 - 1, location.1)),
            None => None,
        };

        // return current location
        Some(location)
    }
}

#[test]
fn test_iterator_for_house_path() {
    let examples: Vec<(Vec<Direction>, Vec<Location>)> = vec![
        (
            vec![Direction::East],
            vec![(0, 0), (1, 0)],
        ),
        (
            vec![Direction::North, Direction::East,
                 Direction::South, Direction::West],
            vec![(0, 0), (0, 1), (1, 1), (1, 0), (0, 0)],
        ),
        (
            vec![Direction::North, Direction::South,
                 Direction::North, Direction::South,
                 Direction::North, Direction::South,
                 Direction::North, Direction::South,
                 Direction::North, Direction::South],
            vec![(0, 0), (0, 1), (0, 0), (0, 1), (0, 0), (0, 1),
                 (0, 0), (0, 1), (0, 0), (0, 1), (0, 0)],
        ),
    ];

    for (directions, reference_locations) in examples.into_iter() {
        let house_path = HousePath{
            location: Some((0, 0)),
            directions: directions.iter(),
        };

        let locations: Vec<_> = house_path.collect();
        assert_eq!(locations, reference_locations);
    }
}

fn parser(c: char) -> Option<Direction> {
    match c {
        '^' => Some(Direction::North),
        'v' => Some(Direction::South),
        '>' => Some(Direction::East),
        '<' => Some(Direction::West),
        _ => None
    }
}

#[test]
fn test_parser() {
    let examples: Vec<(&str, Vec<Direction>)> = vec![
        (
            ">",
            vec![Direction::East],
        ),
        (
            "^>v<",
            vec![Direction::North, Direction::East,
                 Direction::South, Direction::West],
        ),
        (
            "^v^v^v^v^v",
            vec![Direction::North, Direction::South,
                 Direction::North, Direction::South,
                 Direction::North, Direction::South,
                 Direction::North, Direction::South,
                 Direction::North, Direction::South],
        ),
    ];

    for (string, reference_directions) in examples.into_iter() {
        let directions: Vec<Direction> = string.chars()
            .filter_map(parser).collect();
        assert_eq!(directions, reference_directions);
    }
}

fn main() {
    let mut stdin = io::stdin();
    let mut buf = String::new();

    stdin.read_to_string(&mut buf).unwrap();

    // the below does not work due to a type mismatch
    // (expected struct `core::slice::Iter`,
    //     found struct `core::iter::FilterMap`) [E0308]

    // let house_path = HousePath {
    //     directions: buf.chars().filter_map(parser),
    //     location: Some((0, 0)),
    // };
}
