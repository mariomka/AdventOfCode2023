pub fn part1(input: &Vec<&str>) -> u64 {
    let times = input
        .get(0)
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    let distances = input
        .get(1)
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    times
        .iter()
        .zip(distances.iter())
        .map(|(&time, &distance)| calc_record_beating_ways(time, distance))
        .product()
}

pub fn part2(input: &Vec<&str>) -> u64 {
    let time = input
        .get(0)
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .replace(" ", "")
        .parse::<u64>()
        .unwrap();

    let distance = input
        .get(1)
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .replace(" ", "")
        .parse::<u64>()
        .unwrap();

    calc_record_beating_ways(time, distance)
}

fn calc_record_beating_ways(time: u64, distance: u64) -> u64 {
    let from =
        ((time as f64 - (((time * time) - (4 * distance)) as f64).sqrt()) / 2.0).floor() as u64;
    let to = ((time as f64 + (((time * time) - (4 * distance)) as f64).sqrt()) / 2.0).ceil() as u64;

    to - from - 1
}

#[cfg(test)]
mod tests {
    use helpers::input_lines;

    use super::*;

    fn input<'a>() -> Vec<&'a str> {
        let input = "
Time:      7  15   30
Distance:  9  40  200";
        input_lines(input)
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input()), 288)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input()), 71503)
    }
}
