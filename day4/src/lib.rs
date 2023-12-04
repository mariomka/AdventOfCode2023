use std::collections::{HashMap, HashSet};

pub fn part1(input: &Vec<&str>) -> usize {
    input
        .iter()
        .map(|line| {
            let winning_count = winning_count(line);

            if winning_count == 0 {
                return 0;
            }

            1 << (winning_count - 1)
        })
        .sum()
}

pub fn part2(input: &Vec<&str>) -> usize {
    input
        .iter()
        .enumerate()
        .fold(
            HashMap::new(),
            |mut scratchcards: HashMap<usize, usize>, (card_id, line)| {
                let winning_count = winning_count(line);

                let count = *scratchcards
                    .entry(card_id)
                    .and_modify(|scratchcard| *scratchcard += 1)
                    .or_insert(1);

                for i in 1..=winning_count {
                    scratchcards
                        .entry(card_id + i)
                        .and_modify(|scratchcard| *scratchcard += count)
                        .or_insert(count);
                }

                scratchcards
            },
        )
        .values()
        .sum()
}

fn winning_count(line: &&str) -> usize {
    let (_, card) = line.split_once(": ").unwrap();
    let (winning, numbers) = card.split_once("| ").unwrap();

    let winning = winning
        .split(" ")
        .filter_map(|number| number.parse::<usize>().ok())
        .collect::<HashSet<usize>>();

    let numbers = numbers
        .split(" ")
        .filter_map(|number| number.parse::<usize>().ok())
        .collect::<HashSet<usize>>();

    winning.intersection(&numbers).count()
}

#[cfg(test)]
mod tests {
    use helpers::input_lines;

    use super::*;

    fn input<'a>() -> Vec<&'a str> {
        let input = "
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        input_lines(input)
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input()), 13)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input()), 30)
    }
}
