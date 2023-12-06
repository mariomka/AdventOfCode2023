use std::collections::HashMap;
use std::ops::Range;

pub fn part1(input: &Vec<&str>) -> u64 {
    let mut next_ids = input[0]
        .split_once(": ")
        .unwrap()
        .1
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .enumerate()
        .collect::<HashMap<usize, u64>>();

    let mut ids = HashMap::new();

    for line in input[1..].iter() {
        if line.is_empty() {
            continue;
        }

        if line.contains(':') {
            for (&i, &id) in ids.iter() {
                next_ids.entry(i).or_insert(id);
            }

            ids = next_ids.clone();
            next_ids = HashMap::new();

            continue;
        }

        let numbers = line
            .split_whitespace()
            .map(|s| s.parse::<u64>().unwrap())
            .collect::<Vec<u64>>();

        let source = numbers[1]..numbers[1] + numbers[2];
        let destination = numbers[0]..numbers[0] + numbers[2];

        for (i, &id) in ids.iter() {
            if next_ids.contains_key(i) {
                continue;
            }

            if id >= source.start && id < source.end {
                let new_id = id - source.start + destination.start;

                next_ids.insert(*i, new_id);
            }
        }
    }

    for (&i, &id) in ids.iter() {
        next_ids.entry(i).or_insert(id);
    }

    *next_ids.values().min().unwrap()
}

pub fn part2(input: &Vec<&str>) -> u64 {
    let seeds = input[0]
        .split_once(": ")
        .unwrap()
        .1
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    let seed_ranges = seeds
        .chunks(2)
        .map(|chunk| chunk[0]..chunk[0] + chunk[1])
        .collect::<Vec<Range<u64>>>();

    let mut ranges = seed_ranges.clone();
    let mut next_ranges = Vec::new();

    for line in input[1..].iter() {
        if line.is_empty() {
            continue;
        }

        if line.contains(':') {
            ranges.append(&mut next_ranges);
            continue;
        }

        let numbers = line
            .split_whitespace()
            .map(|s| s.parse::<u64>().unwrap())
            .collect::<Vec<u64>>();

        let source = numbers[1]..numbers[1] + numbers[2];
        let destination = numbers[0]..numbers[0] + numbers[2];
        let transpose = |number: u64| number - source.start + destination.start;

        let mut new_ranges = Vec::new();

        for &Range { start, end } in ranges.iter() {
            if start < source.start {
                if end <= source.start {
                    new_ranges.push(start..end);
                    continue;
                }

                if end <= source.end {
                    new_ranges.push(start..source.start);
                    next_ranges.push(destination.start..transpose(end));
                    continue;
                }

                if end > source.end {
                    new_ranges.push(start..source.start);
                    next_ranges.push(destination.start..destination.end);
                    new_ranges.push(source.end..end);
                    continue;
                }
            }

            if start >= source.start && start < source.end {
                if end <= source.end {
                    next_ranges.push(transpose(start)..transpose(end));
                    continue;
                }

                if end > source.end {
                    next_ranges.push(transpose(start)..destination.end);
                    new_ranges.push(source.end..end);
                    continue;
                }
            }

            if start >= source.end {
                new_ranges.push(start..end);
                continue;
            }
        }

        ranges = new_ranges.clone();
    }

    ranges.append(&mut next_ranges);
    ranges.iter().map(|range| range.start).min().unwrap()
}

#[cfg(test)]
mod tests {
    use helpers::input_lines;

    use super::*;

    fn input<'a>() -> Vec<&'a str> {
        let input = "
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        input_lines(input)
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input()), 35)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input()), 46)
    }
}
