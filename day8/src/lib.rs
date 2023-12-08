use std::collections::HashMap;

pub fn part1(input: &Vec<&str>) -> usize {
    let (instructions, nodes) = parse_input(input);

    let mut current = "AAA";

    let mut i = 0;
    while current != "ZZZ" {
        let instruction = instructions[i % instructions.len()];

        if instruction == 'L' {
            current = nodes.get(current).unwrap().0;
        } else {
            current = nodes.get(current).unwrap().1;
        }

        i += 1;
    }

    i
}

pub fn part2(input: &Vec<&str>) -> usize {
    let (instructions, nodes) = parse_input(input);

    let mut current = nodes
        .keys()
        .filter(|name| name.ends_with("A"))
        .map(|&name| name)
        .collect::<Vec<&str>>();

    let paths_count = current.len();
    let mut steps_to_end = Vec::new();

    let mut i = 0;
    while steps_to_end.len() != paths_count {
        let instruction = instructions[i % instructions.len()];
        i += 1;

        current = current
            .iter()
            .map(|name| {
                let node = nodes.get(name).unwrap();

                if instruction == 'L' {
                    node.0
                } else {
                    node.1
                }
            })
            .filter(|name| {
                if name.ends_with("Z") {
                    steps_to_end.push(i);

                    return false;
                }

                true
            })
            .collect();
    }

    steps_to_end
        .iter()
        .skip(1)
        .fold(*steps_to_end.first().unwrap(), |acc, &x| {
            least_common_multiple(acc, x)
        })
}

fn parse_input<'a>(input: &Vec<&'a str>) -> (Vec<char>, HashMap<&'a str, (&'a str, &'a str)>) {
    let mut iter = input.iter();

    let instructions = iter.next().unwrap().chars().collect::<Vec<char>>();

    let nodes = iter
        .map(|line| {
            let (name, values) = line.split_once(" = ").unwrap();

            let (left, right) = values
                .trim_start_matches('(')
                .trim_end_matches(')')
                .split_once(", ")
                .unwrap();

            (name, (left, right))
        })
        .collect::<HashMap<&str, (&str, &str)>>();

    (instructions, nodes)
}

fn least_common_multiple(a: usize, b: usize) -> usize {
    a * b / greatest_common_divisor(a, b)
}

fn greatest_common_divisor(a: usize, b: usize) -> usize {
    let (mut max, mut min) = if a > b { (a, b) } else { (b, a) };

    loop {
        let remainder = max % min;

        if remainder == 0 {
            return min;
        }

        max = min;
        min = remainder;
    }
}

#[cfg(test)]
mod tests {
    use helpers::input_lines;

    use super::*;

    #[test]
    fn test_part1() {
        let input = input_lines(
            "
LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)",
        );

        assert_eq!(part1(&input), 6)
    }

    #[test]
    fn test_part2() {
        let input = input_lines(
            "
LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)",
        );

        assert_eq!(part2(&input), 6)
    }
}
