use helpers::{Coord, Grid};

pub fn part1(input: &Grid<char>) -> usize {
    calc_expanded(input, 1)
}

pub fn part2(input: &Grid<char>, expand: usize) -> usize {
    calc_expanded(input, expand)
}

fn distance(a: &Coord, b: &Coord) -> usize {
    ((a.0 as isize - b.0 as isize).abs() + (a.1 as isize - b.1 as isize).abs()) as usize
}

fn calc_expanded(input: &Grid<char>, expand: usize) -> usize {
    let galaxies: Vec<Coord> = input
        .iter()
        .filter(|(_, char)| **char == '#')
        .map(|(coord, _)| coord)
        .collect();

    let mut expanded_columns: Vec<usize> = vec![];
    let mut expanded_rows: Vec<usize> = vec![];

    for column in 0..input.size.0 {
        if !galaxies.iter().any(|coord| coord.0 == column) {
            expanded_columns.push(column);
        }
    }

    for row in 0..input.size.1 {
        if !galaxies.iter().any(|coord| coord.1 == row) {
            expanded_rows.push(row);
        }
    }

    let expanded_galaxies: Vec<Coord> = galaxies
        .iter()
        .map(|coord| {
            (
                coord.0
                    + expanded_columns
                        .iter()
                        .filter(|&column| *column < coord.0)
                        .count()
                        * expand,
                coord.1 + expanded_rows.iter().filter(|&row| *row < coord.1).count() * expand,
            )
        })
        .collect();

    expanded_galaxies
        .iter()
        .map(|coord| {
            expanded_galaxies
                .iter()
                .filter(|&other| other != coord)
                .map(|other| distance(coord, other))
                .sum::<usize>()
        })
        .sum::<usize>()
        / 2
}

#[cfg(test)]
mod tests {
    use helpers::input_grid;

    use super::*;

    fn input<'a>() -> Grid<char> {
        let input = "
...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
        input_grid(input)
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input()), 374)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input(), 10 - 1), 1030);
        assert_eq!(part2(&input(), 100 - 1), 8410);
    }
}
