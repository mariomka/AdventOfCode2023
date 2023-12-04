use helpers::{input_lines, run};

fn main() {
    let input: Vec<&str> = input_lines(include_str!("../input.txt"));

    run("part1", || day4::part1(&input));
    run("part2", || day4::part2(&input));
}
