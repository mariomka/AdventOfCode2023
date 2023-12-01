pub fn part1(input: &Vec<&str>) -> usize {
    input
        .iter()
        .map(|line| {
            let mut string = String::from("");
            string.push(line.chars().find(|c| c.is_digit(10)).unwrap());
            string.push(line.chars().rev().find(|c| c.is_digit(10)).unwrap());
            string.parse::<usize>().unwrap()
        })
        .sum()
}

pub fn part2(input: &Vec<&str>) -> usize {
    input
        .iter()
        .map(|line| {
            let line = line
                .replace("one", "one1one")
                .replace("two", "two2two")
                .replace("three", "three3three")
                .replace("four", "four4four")
                .replace("five", "five5five")
                .replace("six", "six6six")
                .replace("seven", "seven7seven")
                .replace("eight", "eight8eight")
                .replace("nine", "nine9nine");

            let mut string = String::from("");
            string.push(line.chars().find(|c| c.is_digit(10)).unwrap());
            string.push(line.chars().rev().find(|c| c.is_digit(10)).unwrap());

            string.parse::<usize>().unwrap()
        })
        .sum()
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
