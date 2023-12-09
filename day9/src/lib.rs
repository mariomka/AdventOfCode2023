use itertools::Itertools;

pub fn part1(input: &Vec<&str>) -> isize {
    input
        .iter()
        .map(|line| {
            calc_sequence(line)
                .iter()
                .map(|vec| vec.last().unwrap())
                .sum::<isize>()
        })
        .sum()
}

pub fn part2(input: &Vec<&str>) -> isize {
    input
        .iter()
        .map(|line| {
            calc_sequence(line)
                .iter()
                .rev()
                .fold(0, |acc, vec| vec.first().unwrap() - acc)
        })
        .sum()
}

fn calc_sequence(line: &str) -> Vec<Vec<isize>> {
    let numbers: Vec<isize> = line
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let mut sequence = Vec::new();

    sequence.push(numbers);

    while sequence.last().unwrap().iter().any(|&x| x != 0) {
        sequence.push(
            sequence
                .last()
                .unwrap()
                .iter()
                .tuple_windows()
                .map(|(a, b)| b - a)
                .collect::<Vec<isize>>(),
        );
    }

    sequence
}

#[cfg(test)]
mod tests {
    use helpers::input_lines;

    use super::*;

    fn input<'a>() -> Vec<&'a str> {
        let input = "
0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        input_lines(input)
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input()), 114)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input()), 2)
    }
}
