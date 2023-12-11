use helpers::{Coord, Grid};

fn connections(grid: &Grid<char>, char: char, coord: Coord) -> Vec<Coord> {
    let top = (coord.0 as isize, coord.1 as isize - 1);
    let right = (coord.0 as isize + 1, coord.1 as isize);
    let bottom = (coord.0 as isize, coord.1 as isize + 1);
    let left = (coord.0 as isize - 1, coord.1 as isize);

    match char {
        '|' => {
            vec![top, bottom]
        }
        '-' => {
            vec![left, right]
        }
        'L' => {
            vec![top, right]
        }
        'J' => {
            vec![top, left]
        }
        '7' => {
            vec![bottom, left]
        }
        'F' => {
            vec![bottom, right]
        }
        'S' => {
            vec![top, bottom, left, right]
        }
        _ => {
            vec![]
        }
    }
    .iter()
    .filter(|&&coord| {
        if let Some(char) = grid.maybe_get(coord) {
            return *char != '.';
        }

        false
    })
    .map(|&coord| (coord.0 as usize, coord.1 as usize))
    .collect()
}

#[derive(Clone, Debug)]
struct Node {
    char: char,
    connections: Vec<Coord>,
}

pub fn part1(input: &Grid<char>) -> usize {
    let start = input.iter().find(|(_, char)| **char == 'S').unwrap();
    let mut path = vec![start.0];
    let mut next_connections = connections(input, *start.1, start.0);

    loop {
        let next_coord = next_connections
            .iter()
            .find(|&&next| path.contains(&next) == false);

        if next_coord.is_none() {
            break;
        }

        let next_coord = *next_coord.unwrap();
        let next_char = input.get(next_coord);

        path.push(next_coord);
        next_connections = connections(input, *next_char, next_coord);
    }

    (path.len() as f32 / 2.0).ceil() as usize
}

pub fn part2(input: &Grid<char>) -> usize {
    // TODO
    0
}

#[cfg(test)]
mod tests {
    use helpers::{input_grid, Grid};

    use super::*;

    fn input<'a>() -> Grid<char> {
        let input = "
..F7.
.FJ|.
SJ.L7
|F--J
LJ...";
        input_grid(input)
    }

    #[test]
    fn test_part1() {
        let input = "
..F7.
.FJ|.
SJ.L7
|F--J
LJ...";

        assert_eq!(part1(&input_grid(input)), 8)
    }

    #[test]
    fn test_part2() {
        let input = "
FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJIF7FJ-
L---JF-JLJIIIIFJLJJ7
|F|F-JF---7IIIL7L|7|
|FFJF7L7F-JF7IIL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";

        // TODO
        // assert_eq!(part2(&input_grid(input)), 10)
    }
}
