use std::collections::HashMap;

use helpers::Grid;

pub fn part1(input: &Grid<char>) -> usize {
    let mut current_number = "".to_string();
    let mut current_neighbours = Vec::new();
    let mut numbers = Vec::new();

    for (coord, char) in input.iter() {
        if char.is_numeric() {
            current_number.push(*char);
            current_neighbours.append(
                input
                    .neighbors_iter(coord, true)
                    .collect::<Vec<_>>()
                    .as_mut(),
            );
            continue;
        }

        if current_number.len() > 0 {
            let number = current_number.parse::<usize>().unwrap();

            for &(_, neighbor_char) in current_neighbours.iter() {
                if !neighbor_char.is_numeric() && *neighbor_char != '.' {
                    numbers.push(number);
                    break;
                }
            }

            current_number = "".to_string();
            current_neighbours = Vec::new();
        }
    }

    numbers.iter().sum()
}

pub fn part2(input: &Grid<char>) -> usize {
    let mut current_number = "".to_string();
    let mut current_neighbours = Vec::new();
    let mut possible_gears = HashMap::new();

    for (coord, char) in input.iter() {
        if char.is_numeric() {
            current_number.push(*char);
            current_neighbours.append(
                input
                    .neighbors_iter(coord, true)
                    .collect::<Vec<_>>()
                    .as_mut(),
            );
            continue;
        }

        if current_number.len() > 0 {
            let number = current_number.parse::<usize>().unwrap();

            for &(neighbor_coord, neighbor_char) in current_neighbours.iter() {
                if !neighbor_char.is_numeric() && *neighbor_char == '*' {
                    if let Some((count, acc)) = possible_gears.get(&neighbor_coord) {
                        possible_gears.insert(neighbor_coord, (count + 1, acc * number));
                    } else {
                        possible_gears.insert(neighbor_coord, (1, number));
                    }

                    break;
                }
            }

            current_number = "".to_string();
            current_neighbours = Vec::new();
        }
    }

    possible_gears
        .iter()
        .filter(|&(_, &(gear, _))| gear == 2)
        .map(|(_, &(_, number))| number)
        .sum()
}

#[cfg(test)]
mod tests {
    use helpers::input_grid;

    use super::*;

    fn input<'a>() -> Grid<char> {
        let input = "
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        input_grid(input)
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input()), 4361)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input()), 467835)
    }
}
