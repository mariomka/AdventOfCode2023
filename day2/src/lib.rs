use std::collections::HashMap;

pub fn part1(input: &Vec<&str>) -> usize {
    let cubes = HashMap::from([("red", 12usize), ("green", 13usize), ("blue", 14usize)]);

    input
        .iter()
        .filter_map(|line| {
            let (id, game) = line.split_once(": ").unwrap();

            let game_id = id[5..].parse::<usize>().unwrap();

            for sets in game.split("; ") {
                for set in sets.split(", ") {
                    let (count, color) = set.split_once(' ').unwrap();

                    if count.parse::<usize>().unwrap() > *cubes.get(color).unwrap() {
                        return None;
                    }
                }
            }

            Some(game_id)
        })
        .sum()
}

pub fn part2(input: &Vec<&str>) -> usize {
    input
        .iter()
        .filter_map(|line| {
            let (id, game) = line.split_once(": ").unwrap();

            let game_id = id[5..].parse::<usize>().unwrap();

            let mut colors = HashMap::new();

            for sets in game.split("; ") {
                for set in sets.split(", ") {
                    let (count, color) = set.split_once(' ').unwrap();
                    let count = count.parse::<usize>().unwrap();

                    if !colors.contains_key(color) || count > *colors.get(color).unwrap() {
                        colors.insert(color, count);
                    }
                }
            }

            Some(colors.values().product::<usize>())
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use helpers::input_lines;

    use super::*;

    fn input<'a>() -> Vec<&'a str> {
        let input = "
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        input_lines(input)
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input()), 8)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input()), 2286)
    }
}
