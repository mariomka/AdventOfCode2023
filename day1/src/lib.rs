pub fn part1(input: &Vec<&str>) -> usize {
    input.iter().map(|line| find_number(line.to_string())).sum()
}

pub fn part2(input: &Vec<&str>) -> usize {
    let numbers = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    input
        .iter()
        .map(|line| {
            find_number(
                numbers
                    .iter()
                    .enumerate()
                    .fold(line.to_string(), |line, (i, number)| {
                        line.replace(number, &format!("{}{}{}", number, i + 1, number))
                    }),
            )
        })
        .sum()
}

fn find_number(line: String) -> usize {
    let first_number = line.chars().find(|c| c.is_digit(10)).unwrap();
    let last_number = line.chars().rev().find(|c| c.is_digit(10)).unwrap();

    format!("{}{}", first_number, last_number)
        .parse::<usize>()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use helpers::input_lines;

    use super::*;

    #[test]
    fn test_part1() {
        let input = "
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        assert_eq!(part1(&input_lines(input)), 142)
    }

    #[test]
    fn test_part2() {
        let input = "
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        assert_eq!(part2(&input_lines(input)), 281)
    }
}
